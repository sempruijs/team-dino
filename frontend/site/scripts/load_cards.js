async function FetchData(users, type, id) {
        try {
            RenderData(users, type, id);
        } catch (error) {
            console.error("Error fetching user data:", error);
        }
}

// Function to render user cards
function RenderData(cards, type, id) {
    const container = document.getElementById(id);
    container.innerHTML = ''; // Clear any existing content

    if (type === "omgeving"){
        // Loop through each user and create a card
        // onclick="window.location.assign('./omgeving-uitleg.html?id=${card.title}')"
        cards.forEach(card => {
            const cardTemplate = `
                <div class="column">
                    <a href="./more-info.html?id=${card.title}&type=${type}" style="text-decoration: none; color: #000000;">
                    <div class="card">
                        <img src="./images/${card.image}" alt="${card.alt}">
                        <h3>${card.title}</h3>
                        <a>${card.adress}</a><br>
                        <a href="./more-info.html?id=${card.title}&type=${type}">
                            <button>Meer info  <i class="fa-solid fa-arrow-right"></i></button>
                        </a>
                    </div>
                </div>
            `;
            container.innerHTML += cardTemplate;
        });
    }

    if (type === "faciliteiten"){
        // Loop through each user and create a card
        cards.forEach(card => {
            const cardTemplate = `
                <div class="column">
                    <a href="./more-info.html?id=${card.title}&type=${type}" style="text-decoration: none; color: #000000;">
                        <div class="card">
                            <img src="./images/${card.image}" alt="${card.alt}">
                            <h3>${card.title}</h3>
                            <a href="./more-info.html?id=${card.title}&type=${type}">
                                <button>Meer info  <i class="fa-solid fa-arrow-right"></i></button>
                            </a>
                        </div>
                    </a>
                </div> 
            `;
            container.innerHTML += cardTemplate;
        });
    }

    if (type === "activiteiten"){
        // Loop through each user and create a card
        cards.forEach(card => {
            const cardTemplate = `
                <div class="column">
                    <a href="./more-info.html?id=${card.title}&type=${type}" style="text-decoration: none; color: #000000;">
                        <div class="card">
                            <img src="./images/${card.image}" alt="${card.alt}">
                            <h3>${card.title}</h3>
                            <a href="./more-info.html?id=${card.title}&type=${type}">
                                <button>Meer info  <i class="fa-solid fa-arrow-right"></i></button>
                            </a>
                        </div>
                    </a>
                </div>  
            `;
            container.innerHTML += cardTemplate;
        });
    }

    if (type === "overnachten") {
        // Loop through each user and create a card with a slideshow
        cards.forEach(card => {
            // Split the `images` property into an array
            const imageList = card.images.split(',').map(image => image.trim());
            
            // Create slideshow slides
            const slides = imageList.map((image, index) => `
                <div class="mySlides fade2" style="display: ${index === 0 ? 'block' : 'none'};">
                    <div class="numbertext">${index + 1} / ${imageList.length}</div>
                    <img src="./images/${image}" alt="${card.alt}" style="width:100%">
                </div>
            `).join('');
            
            // Create navigation dots
            const dots = imageList.map((_, index) => `
                <span class="dot${index === 0 ? ' active' : ''}" onclick="currentSlide(${index + 1}, '${card.alt}')"></span>
            `).join('');
        
            // Create the slideshow container
            const slideshowTemplate = `
                <div class="slideshow-container" id="slideshow-${card.alt}">
                    ${slides}
                    <a class="prev" onclick="plusSlides(-1, '${card.alt}')">❮</a>
                    <a class="next" onclick="plusSlides(1, '${card.alt}')">❯</a>
                </div>
                <div style="text-align:center">${dots}</div>
            `;
        
            // Create the card template
            const cardTemplate = `
                <div class="column">
                    <a href="./more-info.html?id=${card.title}&type=${type}" style="text-decoration: none; color: #000000;">
                        <div class="card">
                            ${slideshowTemplate}
                            <h3>${card.title}</h3>
                            <a href="./more-info.html?id=${card.title}&type=${type}">
                                <button>Meer info  <i class="fa-solid fa-arrow-right"></i></button>
                            </a>
                        </div>
                    </a>
                </div> 
            `;
            
            container.innerHTML += cardTemplate;
        });
    }
    
}
