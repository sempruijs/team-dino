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
                    <a href="./index.html#"><img src="./images/logo-placeholder.png" alt="De Groene Weide Logo"></a>
                </div>
                <nav class="navbar">
                    <ul>
                        <li><a href="./index.html#" class="hover-underline-animation">Home</a></li>
                        <li><a href="./deeper.html?page=bookings" class="hover-underline-animation">Overnachten</a></li>
                        <li><a href="./deeper.html?page=facilities" class="hover-underline-animation">Faciliteiten</a></li>
                        <li><a href="./deeper.html?page=activities" class="hover-underline-animation">Activiteiten</a></li>
                        <li><a href="./deeper.html?page=neighbourhood" class="hover-underline-animation">Omgeving</a></li>
                        <li><a href="./contact.html" class="hover-underline-animation">Contact & meer</a></li>
                        <li><a href="./playground.html" class="hover-underline-animation zoek-verborgen">Zoek & Boek</a></li>
                    </ul>
                </nav>
                <nav class="navbar-small">
                    <ul>
                        <li><a href="./index.html#"><img src="./images/logo-placeholder.png" alt="De Groene Weide Logo"></a></li>
                        <li><a href="./index.html#" class="hover-underline-animation tooltip1"><i class="fa-solid fa-house"></i><span class="tooltiptext1">Home</span></a></li>
                        <li><a href="./deeper.html?page=bookings" class="hover-underline-animation tooltip2"><i class="fa-solid fa-bed"></i><span class="tooltiptext2">Overnachten</span></a></li>
                        <li><a href="./deeper.html?page=facilities" class="hover-underline-animation tooltip3"><i class="fa-solid fa-utensils"></i><span class="tooltiptext3">Faciliteiten</span></a></li>
                        <li><a href="./deeper.html?page=activities" class="hover-underline-animation tooltip7"><i class="fa-solid fa-gamepad"></i><span class="tooltiptext7">Activiteiten</span></a></li>
                        <li><a href="./deeper.html?page=neighbourhood" class="hover-underline-animation tooltip4"><i class="fa-solid fa-map"></i><span class="tooltiptext4">Omgeving</span></a></li>
                        <li><a href="./contact.html#" class="hover-underline-animation tooltip5"><i class="fa-solid fa-phone"></i><span class="tooltiptext5">Contact & meer</span></a></li>
                        <li><a href="./playground.html#" class="hover-underline-animation zoek-verborgen tooltip6" ><i class="fa-solid fa-magnifying-glass"></i>  &  <i class="fa-solid fa-calendar-days"></i><span class="tooltiptext6">Zoek & Boek</span></a></li>
                    </ul>
                </nav>
                <div class="book">
                    <a href="./playground.html" class="hover-underline-animation">Zoek & Boek</a>
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

