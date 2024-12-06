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