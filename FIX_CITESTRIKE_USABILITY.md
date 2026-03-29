# FIX: CiteStrike Usability Issues

## Issues to Fix

### 1. No Visible Way to Add PDFs
There is no obvious "Add PDF" button anywhere. New users have no idea how to get papers into the library. The drag-and-drop also does not work.

**Fix:**
- Add a large, prominent "+" button at the top of the "All References" view. Clicking it opens the native file explorer (via Tauri's dialog plugin) filtered to `.pdf` files. Allow selecting multiple PDFs at once.
- Fix drag-and-drop: when user drags files over the app window, show a full-window overlay with a dashed border and text "Drop PDFs here". Wire it to Tauri's drag-drop event. Accept `.pdf` files only.
- Both methods should trigger the same flow: extract DOI → fetch Crossref metadata → add to library.

### 2. No Way to Delete a Reference
Once a paper is added to the library, there is no way to remove it. This is a basic missing feature.

**Fix:**
- Add a delete button (trash icon) on each reference in the All References list. 
- Clicking it shows a confirmation dialog: "Remove [paper title] from library?"
- On confirm, delete the record from SQLite and remove it from the UI list.
- If the paper was cited (cited=1), also remove its cite_order and reset the citation numbering for remaining cited papers.

### 3. Bibliography Requires Manual Building (Should Be Automatic)
Right now, the user has to go to "Build Bibliography", search for papers, manually add them one by one, then copy. This defeats the purpose. Citing a paper should automatically include it in the bibliography.

**Fix:**
- Remove the manual "search and add to bibliography" workflow from the Build Bibliography view.
- Instead, the Build Bibliography view should automatically show ALL papers that the user has clicked "Cite" on (i.e., all papers where cited=1 in the database).
- The view should show:
  - A list of all cited papers in correct order (alphabetical for APA/MLA/Chicago, citation-order for IEEE)
  - A live preview of the formatted bibliography below
  - A "Copy Bibliography" button that copies the entire formatted bibliography to clipboard
  - The citation style dropdown (APA, IEEE, etc.) which re-renders the preview when changed
- No searching, no manual adding. Cite a paper → it is in the bibliography. That is it.

### 4. "Cite" Should Also Show Visual Feedback
When the user clicks "Cite" on a paper, the only feedback is that text is copied to clipboard. There is no indication of what happened.

**Fix:**
- After clicking "Cite", show a brief toast notification: "Copied: (Smith et al., 2023)" or "Copied: [1]"
- Mark the paper in the library list with a small badge or checkmark showing it has been cited
- Show the citation number on the badge for IEEE style (e.g., badge shows "[3]")

## What NOT to Change
- Do not touch the global hotkey (Ctrl+Shift+C) — it works
- Do not touch the Word/PowerPoint add-in — leave it as is
- Do not touch the Crossref search or DOI import — they work
- Do not touch the theme or overall layout
- Do not add hyperlinks or bookmarks to citations — plain text citations are correct
- Do not integrate with Zotero, EndNote, or any external tool

## Acceptance Tests

1. Open app → see a clear "+" button in All References → click it → file explorer opens → select a PDF → paper appears in library with metadata
2. Drag a PDF from desktop onto the app window → overlay appears → drop it → paper appears in library
3. Right-click or click trash icon on a reference → confirmation dialog → confirm → paper is gone
4. Click "Cite" on paper A → toast shows "(Author, Year)" → paste in Notepad → citation text appears
5. Click "Cite" on paper B → now go to Build Bibliography → both papers A and B appear automatically → preview shows formatted bibliography → click "Copy Bibliography" → paste → full bibliography appears
6. Switch style from APA to IEEE → bibliography preview updates → copy and paste → IEEE format
7. Delete paper A from library → go to Build Bibliography → only paper B remains
