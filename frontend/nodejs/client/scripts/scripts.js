let slideIndices = {}; // Track slide index for each slideshow

function plusSlides(n, slideshowId) {
    // Increment or decrement the index
    slideIndices[slideshowId] = (slideIndices[slideshowId] || 1) + n;

    // If the index exceeds the bounds, wrap around
    const slides = document.querySelectorAll(`#slideshow-${slideshowId} .mySlides`);
    if (slideIndices[slideshowId] > slides.length) {
        slideIndices[slideshowId] = 1; // Go to first slide
    } else if (slideIndices[slideshowId] < 1) {
        slideIndices[slideshowId] = slides.length; // Go to last slide
    }

    showSlides(slideshowId);
}

function currentSlide(n, slideshowId) {
    // Set the current slide index
    slideIndices[slideshowId] = n;
    showSlides(slideshowId);
}

function showSlides(slideshowId) {
    // Get all slides and dots for the specified slideshow
    const slides = document.querySelectorAll(`#slideshow-${slideshowId} .mySlides`);
    const dots = document.querySelectorAll(`#slideshow-${slideshowId} ~ div span.dot`);

    // Ensure slideIndices[slideshowId] has a valid value
    if (!slideIndices[slideshowId] || slideIndices[slideshowId] > slides.length) {
        slideIndices[slideshowId] = 1;
    }

    // Hide all slides
    slides.forEach(slide => (slide.style.display = "none"));

    // Remove the active class from all dots
    dots.forEach(dot => dot.className = dot.className.replace(" active", ""));

    // Show the current slide
    slides[slideIndices[slideshowId] - 1].style.display = "block";

    // Add the active class to the corresponding dot (if it exists)
    if (dots[slideIndices[slideshowId] - 1]) {
        dots[slideIndices[slideshowId] - 1].className += " active";
    }
}