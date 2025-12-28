import type { ComponentInfo } from "../components/landing";

export const docsManifest: ComponentInfo[] = [
  {
    "slug": "ascii-renderer",
    "name": "ASCII Renderer",
    "category": "canvas",
    "video": "/previews/ascii-renderer/preview.mp4",
    "poster": "/previews/ascii-renderer/poster.webp",
    "dependencies": {
      "@threlte/core": "^8.3.1",
      "@threlte/extras": "^9.7.1",
      "three": "^0.182.0"
    }
  },
  {
    "slug": "flip-grid",
    "name": "Flip Grid",
    "category": "layout",
    "video": "/previews/flip-grid/preview.mp4",
    "poster": "/previews/flip-grid/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "glass-pane",
    "name": "Glass Pane",
    "category": "canvas",
    "video": "/previews/glass-pane/preview.mp4",
    "poster": "/previews/glass-pane/poster.webp",
    "dependencies": {
      "@threlte/core": "^8.3.1",
      "@threlte/extras": "^9.7.1",
      "three": "^0.182.0"
    }
  },
  {
    "slug": "image-trail",
    "name": "Image Trail",
    "category": "pointer",
    "video": "/previews/image-trail/preview.mp4",
    "poster": "/previews/image-trail/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "infinite-gallery",
    "name": "Infinite Gallery",
    "category": "canvas",
    "video": "/previews/infinite-gallery/preview.mp4",
    "poster": "/previews/infinite-gallery/poster.webp",
    "dependencies": {
      "@threlte/core": "^8.3.1",
      "@threlte/extras": "^9.7.1",
      "three": "^0.182.0"
    }
  },
  {
    "slug": "interactive-grid",
    "name": "Interactive Grid",
    "category": "canvas",
    "video": "/previews/interactive-grid/preview.mp4",
    "poster": "/previews/interactive-grid/poster.webp",
    "dependencies": {
      "@threlte/core": "^8.3.1",
      "@threlte/extras": "^9.7.1",
      "three": "^0.182.0"
    }
  },
  {
    "slug": "logo-carousel",
    "name": "Logo Carousel",
    "category": "showcase",
    "video": "/previews/logo-carousel/preview.mp4",
    "poster": "/previews/logo-carousel/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "macos-dock",
    "name": "MacOS Dock",
    "category": "pointer",
    "video": "/previews/macos-dock/preview.mp4",
    "poster": "/previews/macos-dock/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "magnetic",
    "name": "Magnetic",
    "category": "pointer",
    "video": "/previews/magnetic/preview.mp4",
    "poster": "/previews/magnetic/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "marquee",
    "name": "Marquee",
    "category": "showcase",
    "video": "/previews/marquee/preview.mp4",
    "poster": "/previews/marquee/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "pixelated-image",
    "name": "Pixelated Image",
    "category": "canvas",
    "video": "/previews/pixelated-image/preview.mp4",
    "poster": "/previews/pixelated-image/poster.webp",
    "dependencies": {
      "@threlte/core": "^8.3.1",
      "@threlte/extras": "^9.7.1",
      "three": "^0.182.0"
    }
  },
  {
    "slug": "preloader",
    "name": "Preloader",
    "category": "transition",
    "video": "/previews/preloader/preview.mp4",
    "poster": "/previews/preloader/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "slideshow",
    "name": "Slideshow",
    "category": "showcase",
    "video": "/previews/slideshow/preview.mp4",
    "poster": "/previews/slideshow/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "split-hover",
    "name": "Split Hover",
    "category": "typography",
    "video": "/previews/split-hover/preview.mp4",
    "poster": "/previews/split-hover/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "split-reveal",
    "name": "Split Reveal",
    "category": "typography",
    "video": "/previews/split-reveal/preview.mp4",
    "poster": "/previews/split-reveal/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "text-loop",
    "name": "Text Loop",
    "category": "typography",
    "video": "/previews/text-loop/preview.mp4",
    "poster": "/previews/text-loop/poster.webp",
    "dependencies": {
      "gsap": "^3.14.2"
    }
  },
  {
    "slug": "water-ripple",
    "name": "Water Ripple",
    "category": "canvas",
    "video": "/previews/water-ripple/preview.mp4",
    "poster": "/previews/water-ripple/poster.webp",
    "dependencies": {
      "@threlte/core": "^8.3.1",
      "@threlte/extras": "^9.7.1",
      "three": "^0.182.0"
    }
  }
];

export const getAdjacentDocs = (slug: string) => {
  const index = docsManifest.findIndex((doc) => doc.slug === slug);
  if (index === -1) {
    return { previous: null, next: null };
  }
  const previous = index > 0 ? docsManifest[index - 1] : null;
  const next = index < docsManifest.length - 1 ? docsManifest[index + 1] : null;
  return { previous, next };
};
