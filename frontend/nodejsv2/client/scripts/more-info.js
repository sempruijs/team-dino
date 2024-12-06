async function GetMoreData(type, name) {
    let endpoint = 'http://localhost:3000/api/'.concat(type);
    try {
        // Fetch data from the server
        const response = await fetch(endpoint);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        // Render the fetched data
        Rendercards(data, type, name);
    } catch (error) {
        console.error("Error fetching data:", error);
    }
}

// Function to render user cards
function Rendercards(cards, type, name) {
    if (type === "neighbourhood"){
        // Loop through each user and create a card
        cards.forEach(card => {
            if (card.title === name){
                const container_menu = document.getElementById("booking-menu");
                container_menu.innerHTML = ''; // Clear any existing content
                const menuTemplate = `
                    <div class="background">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                    </div>
                    `;
                container_menu.innerHTML += menuTemplate;

                const container_hero = document.getElementById("hero");
                container_hero.innerHTML = ''; // Clear any existing content
                const heroTemplate = `
                    <div class="hero-text">
                        <h1>${card.title}</h1>
                        <p>${card.text}</p>
                    </div>
                    `;
                container_hero.innerHTML += heroTemplate;

                const container_map = document.getElementById("map");
                container_map.innerHTML = ''; // Clear any existing content
                const mapTemplate = `
                    <div class="map">
                        <div class="contact-info">
                            <h1>informatie van  ${card.title} </h1>
                            <br>
                            <p>adress:  ${card.adress}</p>
                        </div>
                        <div class="map-element">
                            <iframe src="${card.place}" allowfullscreen="" loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
                        </div>
                    </div>
                    `;
                container_map.innerHTML += mapTemplate;
            }
        });
    }

    if (type === "facilities"){
        // Loop through each user and create a card
        cards.forEach(card => {
            if (card.title === name){
                const container_menu = document.getElementById("booking-menu");
                container_menu.innerHTML = ''; // Clear any existing content
                const menuTemplate = `
                    <div class="background">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                    </div>
                    `;
                container_menu.innerHTML += menuTemplate;

                const container_hero = document.getElementById("hero");
                container_hero.innerHTML = ''; // Clear any existing content
                const heroTemplate = `
                    <div class="hero-text">
                        <h1>${card.title}</h1>
                        <p>${card.text}</p>
                        <br>
                        <li>${card.points}</li>
                    </div>
                    `;
                container_hero.innerHTML += heroTemplate;
            }
        });
    }

    if (type === "activities"){
        // Loop through each user and create a card
        cards.forEach(card => {
            if (card.title === name){
                const container_menu = document.getElementById("booking-menu");
                container_menu.innerHTML = ''; // Clear any existing content
                const menuTemplate = `
                    <div class="background">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                    </div>
                    `;
                container_menu.innerHTML += menuTemplate;

                const container_hero = document.getElementById("hero");
                container_hero.innerHTML = ''; // Clear any existing content
                const heroTemplate = `
                    <div class="hero-text">
                        <h1>${card.title}</h1>
                        <br>
                        <p>${card.text}</p>
                    </div>
                    `;
                container_hero.innerHTML += heroTemplate;
            }
        });
    }

    if (type === "bookings"){
        // Loop through each user and create a card
        cards.forEach(card => {
            if (card.title === name){
                const container_menu = document.getElementById("booking-menu");
                container_menu.innerHTML = ''; // Clear any existing content
                const menuTemplate = `
                    <div class="background">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                        <img src="./images/${card.image}" alt="Camping with sunrise" class="background-image">
                    </div>
                    `;
                container_menu.innerHTML += menuTemplate;

                const container_hero = document.getElementById("hero");
                container_hero.innerHTML = ''; // Clear any existing content
                const heroTemplate = `
                    <div class="hero-text">
                        <h1>${card.title}</h1>
                        <p>${card.text}</p>
                        <br>
                        <li>${card.points}</li>
                    </div>
                    `;
                container_hero.innerHTML += heroTemplate;
            }
        });
    }
}
