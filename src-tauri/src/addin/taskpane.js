/* CiteStrike Office Web Add-in — Taskpane Script */

const API_BASE = "http://localhost:27182";
let officeReady = false;
let hostApp = "Unknown"; // "Word" or "Presentation"
let debounceTimer = null;

// ── Office.js Initialization ──────────────────────────────────────

Office.onReady(function (info) {
    officeReady = true;
    hostApp = info.host; // Office.HostType.Word or Office.HostType.PowerPoint

    document.getElementById("searchInput").addEventListener("input", onSearchInput);
    document.getElementById("searchInput").addEventListener("keydown", function (e) {
        if (e.key === "Enter") {
            e.preventDefault();
            doSearch();
        }
    });

    checkConnection();
});

// ── Connection Check ──────────────────────────────────────────────

async function checkConnection() {
    var dot = document.getElementById("connectionStatus");
    var label = document.getElementById("connectionLabel");
    try {
        var resp = await fetch(API_BASE + "/api/health");
        if (resp.ok) {
            dot.className = "connection-dot connected";
            label.textContent = "Connected to CiteStrike";
            // Load all references on connect
            doSearch();
        } else {
            throw new Error("not ok");
        }
    } catch (e) {
        dot.className = "connection-dot disconnected";
        label.textContent = "CiteStrike not running — please start the app";
        setTimeout(checkConnection, 3000);
    }
}

// ── Search ────────────────────────────────────────────────────────

function onSearchInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(doSearch, 150);
}

async function doSearch() {
    var query = document.getElementById("searchInput").value.trim();
    var resultsDiv = document.getElementById("results");

    try {
        var url = API_BASE + "/api/search?q=" + encodeURIComponent(query);
        var resp = await fetch(url);
        if (!resp.ok) throw new Error("Search failed");
        var refs = await resp.json();
        renderResults(refs);
    } catch (e) {
        resultsDiv.innerHTML = '<div class="empty-state"><p>Failed to reach CiteStrike</p><p class="hint">Make sure the app is running</p></div>';
    }
}

// ── Render ────────────────────────────────────────────────────────

function renderResults(refs) {
    var resultsDiv = document.getElementById("results");

    if (refs.length === 0) {
        resultsDiv.innerHTML = '<div class="empty-state"><p>No references found</p><p class="hint">Try different keywords</p></div>';
        return;
    }

    var html = "";
    for (var i = 0; i < refs.length; i++) {
        var ref = refs[i];
        var year = ref.year ? '<span class="ref-year">' + ref.year + "</span>" : "";
        var journal = ref.journal ? '<span class="ref-journal">' + escapeHtml(ref.journal) + "</span>" : "";

        html +=
            '<div class="ref-card" data-index="' + i + '">' +
                '<div class="ref-title">' + escapeHtml(ref.title) + "</div>" +
                '<div class="ref-authors">' + escapeHtml(ref.authors) + "</div>" +
                '<div class="ref-meta">' + year + journal + "</div>" +
                '<div class="ref-actions">' +
                    '<button class="btn btn-insert" onclick="insertFull(' + i + ')">Insert Citation</button>' +
                    '<button class="btn btn-inline" onclick="insertInline(' + i + ')">In-text</button>' +
                    '<button class="btn btn-copy" onclick="copyToClipboard(' + i + ')">Copy</button>' +
                "</div>" +
            "</div>";
    }

    resultsDiv.innerHTML = html;
    // Stash refs for button handlers
    window._currentRefs = refs;
}

// ── Insert into Document ──────────────────────────────────────────

async function insertFull(index) {
    var ref = window._currentRefs[index];
    var style = document.getElementById("styleSelect").value;

    try {
        var resp = await fetch(API_BASE + "/api/search?q=" + encodeURIComponent(ref.title));
        var refs = await resp.json();
        var fullRef = refs.find(function (r) { return r.id === ref.id; }) || ref;

        var fmtResp = await fetch(
            API_BASE + "/api/format?style=" + encodeURIComponent(style),
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(fullRef),
            }
        );
        var data = await fmtResp.json();
        var text = data.formatted;

        if (hostApp === Office.HostType.Word) {
            await Word.run(function (context) {
                var range = context.document.getSelection();
                range.insertText(text, Word.InsertLocation.replace);
                return context.sync();
            });
        } else if (hostApp === Office.HostType.PowerPoint) {
            // PowerPoint: insert into selected text box
            Office.context.document.setSelectedDataAsync(
                text,
                { coercionType: Office.CoercionType.Text },
                function () {}
            );
        }

        showStatus("Inserted " + style + " citation", "success");
        flashButton(index, "btn-insert");
    } catch (e) {
        showStatus("Insert failed: " + e.message, "error");
    }
}

async function insertInline(index) {
    var ref = window._currentRefs[index];
    var style = document.getElementById("styleSelect").value;

    try {
        var fmtResp = await fetch(
            API_BASE + "/api/format-inline?style=" + encodeURIComponent(style),
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(ref),
            }
        );
        var data = await fmtResp.json();
        var text = data.formatted;

        if (hostApp === Office.HostType.Word) {
            await Word.run(function (context) {
                var range = context.document.getSelection();
                range.insertText(text, Word.InsertLocation.end);
                return context.sync();
            });
        } else {
            Office.context.document.setSelectedDataAsync(
                text,
                { coercionType: Office.CoercionType.Text },
                function () {}
            );
        }

        showStatus("Inserted in-text citation", "success");
        flashButton(index, "btn-inline");
    } catch (e) {
        showStatus("Insert failed: " + e.message, "error");
    }
}

async function copyToClipboard(index) {
    var ref = window._currentRefs[index];
    var style = document.getElementById("styleSelect").value;

    try {
        var fmtResp = await fetch(
            API_BASE + "/api/format?style=" + encodeURIComponent(style),
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(ref),
            }
        );
        var data = await fmtResp.json();
        await navigator.clipboard.writeText(data.formatted);
        showStatus("Copied to clipboard", "success");
        flashButton(index, "btn-copy");
    } catch (e) {
        showStatus("Copy failed: " + e.message, "error");
    }
}

// ── Helpers ───────────────────────────────────────────────────────

function showStatus(msg, type) {
    var el = document.getElementById("status");
    el.textContent = msg;
    el.className = "status " + type;
    setTimeout(function () {
        el.className = "status hidden";
    }, 2500);
}

function flashButton(index, btnClass) {
    var card = document.querySelectorAll(".ref-card")[index];
    if (!card) return;
    var btn = card.querySelector("." + btnClass);
    if (!btn) return;
    btn.classList.add("btn-success");
    setTimeout(function () {
        btn.classList.remove("btn-success");
    }, 1200);
}

function escapeHtml(str) {
    if (!str) return "";
    var div = document.createElement("div");
    div.textContent = str;
    return div.innerHTML;
}
