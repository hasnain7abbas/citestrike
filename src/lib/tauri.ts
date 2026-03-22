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
	created_at: string;
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

export type CitationStyle = 'APA' | 'MLA' | 'Chicago' | 'IEEE' | 'Harvard' | 'BibTeX';

export async function searchReferences(query: string): Promise<Reference[]> {
	return invoke('search_references', { query });
}

export async function addReference(newRef: NewReference): Promise<Reference> {
	return invoke('add_reference', { newRef });
}

export async function deleteReference(id: string): Promise<void> {
	return invoke('delete_reference', { id });
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
