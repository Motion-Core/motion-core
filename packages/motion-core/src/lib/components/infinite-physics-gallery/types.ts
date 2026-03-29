export interface InfinitePhysicsGalleryImageItem {
	/**
	 * Optional stable item identifier.
	 */
	id?: string | number;
	/**
	 * Discriminator for image media items.
	 */
	type: "image";
	/**
	 * Image payload.
	 */
	image: {
		/**
		 * Image source URL or local path.
		 */
		src: string;
		/**
		 * Optional image alt text.
		 */
		alt?: string;
	};
}

export interface InfinitePhysicsGalleryVideoItem {
	/**
	 * Optional stable item identifier.
	 */
	id?: string | number;
	/**
	 * Discriminator for video media items.
	 */
	type: "video";
	/**
	 * Video payload.
	 */
	video: {
		/**
		 * Video source URL or local path.
		 */
		src: string;
	};
}

/**
 * Item union accepted by InfinitePhysicsGallery.
 * Use either the image shape or the video shape.
 */
export type InfinitePhysicsGalleryItem =
	| InfinitePhysicsGalleryImageItem
	| InfinitePhysicsGalleryVideoItem;
