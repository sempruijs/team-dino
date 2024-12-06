async function loadPage() {
    // Check if the current URL has a query string
    const queryString = window.location.search;

    // If there is no query string, redirect to index.html?page=home
    if (!queryString) {
        window.location.href = `${window.location.pathname}?page=home`;
    } else {
        let endpoint = 'http://localhost:3000/api/pages';
        try {
            // Fetch data from the server
            const response = await fetch(endpoint);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const data = await response.json();
            // Render the fetched data
            const urlParams = new URLSearchParams(window.location.search);
            const page = urlParams.get('page');
            RenderPages(data, page);
        } catch (error) {
            console.error("Error fetching data:", error);
        }
        console.log('Page query detected:', queryString);
    }
}

function RenderPages(pages, type) {
    pages.forEach(page => {
        if (page.page === type){
            const container_element = document.getElementById('body');
            container_element.innerHTML = ''; // Clear any existing content
            let elements = page.elements.split(',');
            elements.forEach(element => {
                if (element === 'header' || element === 'footer'){
                    const elementTemplate = `
                    <${element} id="${element}">
                    </${element}>
                    `;
                    container_element.innerHTML += elementTemplate;
                }
                if (element === 'row'){
                    const elementTemplate = `
                    <section id="${element}" class="${element}">
                    </section>
                    `;
                    container_element.innerHTML += elementTemplate;
                    FetchData(type, 'row');
                }
                if (element === 'map'){
                    const elementTemplate = `
                    <section id="${element}" class="${element}">
                    </section>
                    `;
                    container_element.innerHTML += elementTemplate;
                    FetchMainData(element);
                }
                if (element === 'info'){
                    const elementTemplate = `
                    <section id="${element}" class="${element}">
                    </section>
                    `;
                    container_element.innerHTML += elementTemplate;
                    FetchMainData(element);
                }
                if (element === 'info-small'){
                    const elementTemplate = `
                    <section id="${element}" class="${element}">
                    </section>
                    `;
                    container_element.innerHTML += elementTemplate;
                    FetchMainData(element);
                }
                else {
                    const elementTemplate = `
                    <section id="${element}" class="${element}">
                    </section>
                    `;
                    container_element.innerHTML += elementTemplate;
                }
            });
            FetchMainData('header');  
            FetchMainData('booking-menus'); 
            FetchMainData('footer');
            FetchHeroData(type);
        }
    });
}

async function FetchMainData(id) {
    try {
        RenderMenu(id);
    } catch (error) {
        console.error("Error fetching user data:", error);
    }
}

function RenderMenu(id) {
    const container = document.getElementById(id);
    container.innerHTML = ''; // Clear any existing content
    if (id === "header"){
        const cardTemplate = `
                <div class="logo">
                    <a href="?page=home"><img src="./images/logo-placeholder.png" alt="De Groene Weide Logo"></a>
                </div>
                <nav class="navbar">
                    <ul>
                        <li><a href="?page=home" class="hover-underline-animation">Home</a></li>
                        <li><a href="?page=bookings" class="hover-underline-animation">Overnachten</a></li>
                        <li><a href="?page=facilities" class="hover-underline-animation">Faciliteiten</a></li>
                        <li><a href="?page=activities" class="hover-underline-animation">Activiteiten</a></li>
                        <li><a href="?page=neighbourhood" class="hover-underline-animation">Omgeving</a></li>
                        <li><a href="?page=contact" class="hover-underline-animation">Contact & meer</a></li>
                        <li><a href="#" class="hover-underline-animation zoek-verborgen">Zoek & Boek</a></li>
                    </ul>
                </nav>
                <nav class="navbar-small">
                    <ul>
                        <li><a href="?page=home"><img src="./images/logo-placeholder.png" alt="De Groene Weide Logo"></a></li>
                        <li><a href="?page=home" class="hover-underline-animation tooltip1"><i class="fa-solid fa-house"></i><span class="tooltiptext1">Home</span></a></li>
                        <li><a href="?page=bookings" class="hover-underline-animation tooltip2"><i class="fa-solid fa-bed"></i><span class="tooltiptext2">Overnachten</span></a></li>
                        <li><a href="?page=facilities" class="hover-underline-animation tooltip3"><i class="fa-solid fa-utensils"></i><span class="tooltiptext3">Faciliteiten</span></a></li>
                        <li><a href="?page=activities" class="hover-underline-animation tooltip7"><i class="fa-solid fa-gamepad"></i><span class="tooltiptext7">Activiteiten</span></a></li>
                        <li><a href="?page=neighbourhood" class="hover-underline-animation tooltip4"><i class="fa-solid fa-map"></i><span class="tooltiptext4">Omgeving</span></a></li>
                        <li><a href="?page=contact" class="hover-underline-animation tooltip5"><i class="fa-solid fa-phone"></i><span class="tooltiptext5">Contact & meer</span></a></li>
                        <li><a href="#" class="hover-underline-animation zoek-verborgen tooltip6" ><i class="fa-solid fa-magnifying-glass"></i>  &  <i class="fa-solid fa-calendar-days"></i><span class="tooltiptext6">Zoek & Boek</span></a></li>
                    </ul>
                </nav>
                <div class="book">
                    <a href="#" class="hover-underline-animation">Zoek & Boek</a>
                </div>
            `;
        container.innerHTML += cardTemplate;
    }

    if (id === "map"){
        const cardTemplate = `
                <div class="contact-info">
                    <h1>contact info</h1>
                    <br>
                    <p>contact</p>
                </div>
                <div class="map-element">
                    <iframe src="https://www.google.com/maps/embed?pb=!1m14!1m8!1m3!1d1368.916872403317!2d3.5256398085689944!3d51.46866689592991!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c4978c1a642823%3A0x851ef8278504a987!2sRoompot%20Camping%20Dishoek!5e0!3m2!1snl!2snl!4v1732892925130!5m2!1snl!2snl" allowfullscreen="" loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
                </div>
            `;
        container.innerHTML += cardTemplate;
    }

    if (id === "info"){
        const cardTemplate = `
                <div class="camping-section">
                    <div class="text-container">
                        <h2>Kom je met de caravan?</h2>
                        <p>Met privé-sanitair of extra ruime plaatsen</p>
                        <p>Ervaar het gemak van kamperen met je caravan op onze comfortabele plekken, compleet met privé-sanitair (inclusief vloerverwarming!). Voor wie extra ruimte zoekt, hebben we ook XL-plaatsen beschikbaar. Geniet van rust, natuur en alle gemakken tijdens je verblijf!</p>
                    </div>
                    <div class="image-container">
                        <img src="./images/2.jpg" alt="Caravan" class="caravan-image">
                    </div>
                </div>
                <div class="camping-section-right">
                    <div class="image-container">
                        <img src="./images/38.jpg" alt="Tent">
                    </div>
                    <div class="text-container">
                        <h2>Kom je logeren in een tent?</h2>
                        <p>Kies voor een stijlvolle glampingtent of een ruime kampeerplek!</p>
                        <p>Geniet van een unieke glampingervaring in onze luxe ingerichte tenten, compleet met comfortabele bedden en alle gemakken van thuis. Of ga voor een extra ruime kampeerplek met alle rust en ruimte om te genieten van het buitenleven. Welke tent kies jij?</p>
                    </div>
                </div>
                <div class="camping-section">
                    <div class="text-container">
                        <h2>Omgeving</h2>
                        <p>Met privé-sanitair of extra ruime plaatsen</p>
                        <p>Ervaar het gemak van kamperen met je caravan op onze comfortabele plekken, compleet met privé-sanitair (inclusief vloerverwarming!). Voor wie extra ruimte zoekt, hebben we ook XL-plaatsen beschikbaar. Geniet van rust, natuur en alle gemakken tijdens je verblijf!</p>
                    </div>
                    <div class="image-container">
                        <img src="./images/28.jpg" alt="Omgeving">
                    </div>
                </div>
                <div class="camping-section-right">
                    <div class="image-container">
                        <img src="./images/7.jpg" alt="Activiteiten">
                    </div>
                    <div class="text-container">
                        <h2>Activiteiten</h2>
                        <p>Kies voor een stijlvolle glampingtent of een ruime kampeerplek!</p>
                        <p>Geniet van een unieke glampingervaring in onze luxe ingerichte tenten, compleet met comfortabele bedden en alle gemakken van thuis. Of ga voor een extra ruime kampeerplek met alle rust en ruimte om te genieten van het buitenleven. Welke tent kies jij?</p>
                    </div>
                </div>
            `;
        container.innerHTML += cardTemplate;
    }

    if (id === "info-small"){
        const cardTemplate = `
            <div class="camping-section">
                <div class="image-container">
                    <img src="./images/2.jpg" alt="Caravan">
                </div>
                <div class="text-container">
                    <h2>Kom je met de caravan?</h2>
                    <p>Met privé-sanitair of extra ruime plaatsen</p>
                    <p>Ervaar het gemak van kamperen met je caravan op onze comfortabele plekken, compleet met privé-sanitair (inclusief vloerverwarming!). Voor wie extra ruimte zoekt, hebben we ook XL-plaatsen beschikbaar. Geniet van rust, natuur en alle gemakken tijdens je verblijf!</p>
                </div>
            </div>
            <div class="camping-section-right">
                <div class="image-container">
                    <img src="./images/38.jpg" alt="Tent">
                </div>
                <div class="text-container">
                    <h2>Kom je logeren in een tent?</h2>
                    <p>Kies voor een stijlvolle glampingtent of een ruime kampeerplek!</p>
                    <p>Geniet van een unieke glampingervaring in onze luxe ingerichte tenten, compleet met comfortabele bedden en alle gemakken van thuis. Of ga voor een extra ruime kampeerplek met alle rust en ruimte om te genieten van het buitenleven. Welke tent kies jij?</p>
                </div>
            </div>
            <div class="camping-section">
                <div class="image-container">
                    <img src="./images/28.jpg" alt="Omgeving">
                </div>
                <div class="text-container">
                    <h2>Omgeving</h2>
                    <p>Met privé-sanitair of extra ruime plaatsen</p>
                    <p>Ervaar het gemak van kamperen met je caravan op onze comfortabele plekken, compleet met privé-sanitair (inclusief vloerverwarming!). Voor wie extra ruimte zoekt, hebben we ook XL-plaatsen beschikbaar. Geniet van rust, natuur en alle gemakken tijdens je verblijf!</p>
                </div>
            </div>
            <div class="camping-section-right">
                <div class="image-container">
                    <img src="./images/7.jpg" alt="Activiteiten">
                </div>
                <div class="text-container">
                    <h2>Activiteiten</h2>
                    <p>Kies voor een stijlvolle glampingtent of een ruime kampeerplek!</p>
                    <p>Geniet van een unieke glampingervaring in onze luxe ingerichte tenten, compleet met comfortabele bedden en alle gemakken van thuis. Of ga voor een extra ruime kampeerplek met alle rust en ruimte om te genieten van het buitenleven. Welke tent kies jij?</p>
                </div>
            </div>
            `;
        container.innerHTML += cardTemplate;
    }

    if (id === "booking-menu"){
        const cardTemplate = `
            <div class="background">
                <img src="./images/21.jpg" alt="Camping with sunrise" class="background-image">
                <img src="./images/8.jpg" alt="Camping with sunrise" class="background-image">
                <img src="./images/12.jpg" alt="Camping with sunrise" class="background-image">
                <img src="./images/4.jpg" alt="Camping with sunrise" class="background-image">
            </div>
            `;
        container.innerHTML += cardTemplate;
    }

    if (id === "booking-menus"){
        const verblijfTemplate = `
            <label for="stay-type">Verblijfstype:</label>
            <select id="stay-type" name="stay-type">
                <option value="">Maak Uw keuze</option>
                <option value="type1">Type 1</option>
                <option value="type2">Type 2</option>
            </select>
        `

        const periodeTemplate = `
            <label for="period">Periode:</label>
            <select id="periodvan" name="periodvan">
                <option value="">Van: </option>
                <option value="period1">Periode 1</option>
                <option value="period2">Periode 2</option>
            </select>
            <select id="periodtot" name="periodtot">
                <option value="">Tot: </option>
                <option value="period1">Periode 1</option>
                <option value="period2">Periode 2</option>
            </select>
        `

        const cardTemplate = `
            <div class="background">
                <img src="./images/21.jpg" alt="Camping with sunrise" class="background-image">
                <img src="./images/8.jpg" alt="Camping with sunrise" class="background-image">
                <img src="./images/12.jpg" alt="Camping with sunrise" class="background-image">
                <img src="./images/4.jpg" alt="Camping with sunrise" class="background-image">
            </div>
            <div class="background-2">
                <div class="booking-menu">
                    <div class="booking-content">
                        <h3>Boek je vakantie bij Dishoek</h3>
                        <form>
                            <div class="form-row">
                                <label for="persons">Aantal personen:</label>
                                <input type="number" id="persons" name="persons" min="1" value="2">
                            </div>
                            <div class="form-row">
                                ${verblijfTemplate}
                            </div>
                            <div class="form-row">
                                ${periodeTemplate}
                            </div>
                            <button type="submit" class="search-btn">Zoeken</button>
                        </form>
                    </div>
                </div>
            </div>
            `;
        container.innerHTML += cardTemplate;
    }
    
    if (id === "footer"){
        const cardTemplate = `
            <div class="contact">
                <p><a class="contact" href="tel:123456789">Tel: 123456789</a></p>
                <p><a class="contact" href="mailto:info@degroeneweide.nl">Email: info@degroeneweide.nl</a></p>
            </div>
            <div class="address">
                <p><a class="address" href="https://www.google.nl/maps/place/Roompot+Noordzee+R%C3%A9sidence+Dishoek/@51.4737493,3.5211743,17.76z/data=!4m15!1m8!3m7!1s0x47c49792488554ff:0xd0b2b6c4e06cb1b0!2sDishoek,+4371+NR+Koudekerke!3b1!8m2!3d51.4750683!4d3.5232399!16zL20vMGNiMjJi!3m5!1s0x47c4979268ed0ed9:0xd06779989b29c4c2!8m2!3d51.4738164!4d3.5213878!16s%2Fg%2F11c1q406cx?entry=ttu&g_ep=EgoyMDI0MTEyNC4xIKXMDSoASAFQAw%3D%3D" target="_blank">Camping Dishoek</a></p>
            </div>
            `;
        container.innerHTML += cardTemplate;
    }
}

async function FetchData(type, id) {
    let endpoint = 'http://localhost:3000/api/'.concat(type);
    try {
        // Fetch data from the server
        const response = await fetch(endpoint);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        // Render the fetched data
        Rendercards(data, type, id);
    } catch (error) {
        console.error("Error fetching data:", error);
    }
}

// Function to render user cards
function Rendercards(cards, type, id) {
    const container = document.getElementById(id);
    container.innerHTML = ''; // Clear any existing content
    if (type === "neighbourhood"){
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

    if (type === "facilities"){
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

    if (type === "activities"){
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

    if (type === "bookings") {
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

async function FetchHeroData(type) {
    let endpoint = 'http://localhost:3000/api/hero';
    try {
        // Fetch data from the server
        const response = await fetch(endpoint);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        // Render the fetched data
        Renderhero(data, type);
    } catch (error) {
        console.error("Error fetching data:", error);
    }
}

function Renderhero(cards, type) {
    cards.forEach(card => {
        if (card.page === type){
            const container_hero = document.getElementById("hero");
            container_hero.innerHTML = ''; // Clear any existing content
            const heroTemplate = `
                <div class="hero-text">
                    <h1>${card.title}</h1>
                    <p>${card.text}</p>
                </div>
                `;
            container_hero.innerHTML += heroTemplate;
        }
    });
}