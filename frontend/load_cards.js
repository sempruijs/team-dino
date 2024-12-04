// const users = [
//     {   
//         image: "47.jpg",
//         alt: "Tent",
//         title: "Strand van Dishoek",
//         text: "Waai lekker uit of geniet van de zon op het brede zandstrand van Dishoek. Vanaf het strand zie je grote zeeschepen vlak langs de kustlijn varen, een uniek gezicht! Het strand van Dishoek heeft de internationale Blauwe Vlag onderscheiding. Het zwemwater is dus van uitstekende kwaliteit en het zandstrand wordt goed schoongehouden.",
//         adress: "Dishoek 1, Dishoek"
//     },

//     {
//         image: "48.jpg",
//         alt: "Tent",
//         title: "Vuurtorens van Kaapduinen",
//         text: "Wandel door de duinen van Dishoek langs de twee herkenbare gestreepte vuurtorens van Kaapduinen. Vanaf de duinen heb je prachtig uitzicht over het strand en de Noordzee. Spot jij de schepen in de verte al?",
//         adress: "Dishoek 99, Koudekerke"
//     }
// ];


async function FetchData(users, type, id) {
        try {
            renderOmgevingCards(users, type, id);
        } catch (error) {
            console.error("Error fetching user data:", error);
        }
}

// Function to render user cards
function renderOmgevingCards(cards, type, id) {
    const container = document.getElementById(id);
    container.innerHTML = ''; // Clear any existing content

    if (type === "omgeving"){
        // Loop through each user and create a card
        cards.forEach(card => {
            const cardTemplate = `
                <div class="column">
                    <div class="card">
                        <img src="./images/${card.image}" alt="${card.alt}">
                        <h3>${card.title}</h3>
                        <a>${card.text}</a><br> 
                        <a>${card.adress}</a>
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
                    <div class="card">
                        <img src="./images/${card.image}" alt="${card.alt}">
                        <h3>${card.title}</h3>
                        <a>${card.text}</a>
                        <li>${card.points}</li>
                    </div>
                </div> 
            `;
            container.innerHTML += cardTemplate;
        });
    }
}
