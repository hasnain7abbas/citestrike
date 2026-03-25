import { invoke } from '@tauri-apps/api/core';

export interface Reference {
	id: string;
	title: string;
	authors: string;
	year: number | null;
	doi: string | null;
	journal: string | null;
	volume: string | null;
	issue: string | null;
	pages: string | null;
	abstract_text: string | null;
	url: string | null;
	ref_type: string;
	bibtex_key: string;
	folder_id: string | null;
	created_at: string;
	cited: boolean;
	cite_order: number | null;
}

export interface NewReference {
	title: string;
	authors: string;
	year: number | null;
	doi: string | null;
	journal: string | null;
	volume: string | null;
	issue: string | null;
	pages: string | null;
	abstract_text: string | null;
	url: string | null;
	ref_type: string;
}

export interface Folder {
	id: string;
	name: string;
	color: string;
	created_at: string;
}

export type CitationStyle = 'APA' | 'MLA' | 'Chicago' | 'IEEE' | 'Harvard' | 'Vancouver' | 'BibTeX';

export async function searchReferences(query: string, folderId?: string | null): Promise<Reference[]> {
	return invoke('search_references', { query, folderId: folderId ?? null });
}

export async function addReference(newRef: NewReference, folderId?: string | null): Promise<Reference> {
	return invoke('add_reference', { newRef, folderId: folderId ?? null });
}

export async function deleteReference(id: string): Promise<void> {
	return invoke('delete_reference', { id });
}

export async function moveReference(refId: string, folderId?: string | null): Promise<void> {
	return invoke('move_reference', { refId, folderId: folderId ?? null });
}

export async function formatRef(reference: Reference, style: CitationStyle): Promise<string> {
	return invoke('format_ref', { reference, style });
}

export async function fetchDoi(doi: string): Promise<NewReference> {
	return invoke('fetch_doi', { doi });
}

export async function searchOnline(query: string): Promise<NewReference[]> {
	return invoke('search_online', { query });
}

export async function importPdf(path: string): Promise<string> {
	return invoke('import_pdf', { path });
}

export async function createFolder(name: string, color: string): Promise<Folder> {
	return invoke('create_folder', { name, color });
}

export async function getFolders(): Promise<Folder[]> {
	return invoke('get_folders');
}

export async function renameFolder(id: string, name: string): Promise<void> {
	return invoke('rename_folder', { id, name });
}

export async function deleteFolder(id: string): Promise<void> {
	return invoke('delete_folder', { id });
}

export async function getRefCount(folderId?: string | null): Promise<number> {
	return invoke('get_ref_count', { folderId: folderId ?? null });
}

export async function formatInlineCitation(reference: Reference, style: CitationStyle, number?: number): Promise<string> {
	return invoke('format_inline_citation', { reference, style, number: number ?? null });
}

export async function formatBatchBibliography(references: Reference[], style: CitationStyle): Promise<string> {
	return invoke('format_batch_bibliography', { references, style });
}

// Rich text clipboard (RTF — preserves italics in Word/PPT)
export async function copyRichCitation(reference: Reference, style: CitationStyle): Promise<void> {
	return invoke('copy_rich_citation', { reference, style });
}

export async function copyRichInline(reference: Reference, style: CitationStyle, number?: number): Promise<void> {
	return invoke('copy_rich_inline', { reference, style, number: number ?? null });
}

export async function copyRichBibliography(references: Reference[], style: CitationStyle): Promise<void> {
	return invoke('copy_rich_bibliography', { references, style });
}

// Direct Word/PowerPoint insertion
export async function insertCitationIntoWord(reference: Reference, style: CitationStyle): Promise<string> {
	return invoke('insert_citation_into_word', { reference, style });
}

export async function insertInlineIntoWord(reference: Reference, style: CitationStyle, number?: number): Promise<string> {
	return invoke('insert_inline_into_word', { reference, style, number: number ?? null });
}

export async function insertCitationIntoPpt(reference: Reference, style: CitationStyle): Promise<string> {
	return invoke('insert_citation_into_ppt', { reference, style });
}

export async function insertBibliographyIntoWord(references: Reference[], style: CitationStyle): Promise<string> {
	return invoke('insert_bibliography_into_word', { references, style });
}

// --- Citation workflow ---

/** Mark a reference as cited, copy in-text citation to clipboard. Returns the inline text. */
export async function citeReference(id: string, style: CitationStyle): Promise<string> {
	return invoke('cite_reference', { id, style });
}

/** Unmark a reference as cited. */
export async function unciteReference(id: string): Promise<void> {
	return invoke('uncite_reference', { id });
}

/** Get all cited references in cite order. */
export async function getCitedReferences(): Promise<Reference[]> {
	return invoke('get_cited_references');
}

/** Copy the bibliography of all cited papers. Returns plain text preview. */
export async function copyCitedBibliography(style: CitationStyle): Promise<string> {
	return invoke('copy_cited_bibliography', { style });
}

/** Reset all citation marks. */
export async function resetCitations(): Promise<void> {
	return invoke('reset_citations');
}

/** Update an existing reference's metadata. */
export async function updateReference(id: string, updated: NewReference): Promise<void> {
	return invoke('update_reference', { id, updated });
}

/** Write BibTeX export to the given file path. */
export async function writeBibtexFile(path: string): Promise<void> {
	return invoke('write_bibtex_file', { path });
}
