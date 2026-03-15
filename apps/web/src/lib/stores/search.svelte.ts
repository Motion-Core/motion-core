export class SearchState {
	isOpen = $state(false);

	toggle() {
		this.isOpen = !this.isOpen;
	}

	open() {
		this.isOpen = true;
	}

	close() {
		this.isOpen = false;
	}
}

export const searchState = new SearchState();
