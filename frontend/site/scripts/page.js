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
                        <li><a href="./overnachten.html#" class="hover-underline-animation">Overnachten</a></li>
                        <li><a href="./faciliteiten.html" class="hover-underline-animation">Faciliteiten</a></li>
                        <li><a href="./activiteiten.html" class="hover-underline-animation">Activiteiten</a></li>
                        <li><a href="./omgeving.html" class="hover-underline-animation">Omgeving</a></li>
                        <li><a href="./contact.html" class="hover-underline-animation">Contact & meer</a></li>
                        <li><a href="./acount.html" class="hover-underline-animation">Acount</a></li>
                        <li><a href="./playground.html" class="hover-underline-animation zoek-verborgen">Zoek & Boek</a></li>
                    </ul>
                </nav>
                <nav class="navbar-small">
                    <ul>
                        <li><a href="./index.html#"><img src="./images/logo-placeholder.png" alt="De Groene Weide Logo"></a></li>
                        <li><a href="./index.html#" class="hover-underline-animation tooltip1"><i class="fa-solid fa-house"></i><span class="tooltiptext1">Home</span></a></li>
                        <li><a href="./overnachten.html#" class="hover-underline-animation tooltip2"><i class="fa-solid fa-bed"></i><span class="tooltiptext2">Overnachten</span></a></li>
                        <li><a href="./faciliteiten.html#" class="hover-underline-animation tooltip3"><i class="fa-solid fa-utensils"></i><span class="tooltiptext3">Faciliteiten</span></a></li>
                        <li><a href="./activiteiten.html#" class="hover-underline-animation tooltip7"><i class="fa-solid fa-gamepad"></i><span class="tooltiptext7">Activiteiten</span></a></li>
                        <li><a href="./omgeving.html#" class="hover-underline-animation tooltip4"><i class="fa-solid fa-map"></i><span class="tooltiptext4">Omgeving</span></a></li>
                        <li><a href="./contact.html#" class="hover-underline-animation tooltip5"><i class="fa-solid fa-phone"></i><span class="tooltiptext5">Contact & meer</span></a></li>
                        <li><a href="./acount.html#" class="hover-underline-animation tooltip8"><i class="fa-solid fa-user"></i><span class="tooltiptext8">Acount</span></a></li>
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
            <label for="period1">Periode:</label>
            <select id="periodvan" name="periodvan">
                <option value="">Van: </option>
                <option value="period1"><input type="date"></option>
                <option value="period2"><input type="date"></option>
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
    if (id === "contact"){
        const cardTemplate = `
            <div class="contact-info">
                <h1>contact info</h1>
                 <br>
                  <p>telefoonnummer: 06-12345678\n</p>
                 <p>e-mail adress: example.email@email.com\n<p>
                 <p>adress: Dishoek 2\n<p>
                 <p>postcode: 4371 NT Koudekerke\n<p>
                 <h2>openingstijden receptie</h2>
                 <p>Maandag 09:00 tot 17:00\n<p>
                 <p>Dinsdag 09:00 tot 13:00\n<p>
                 <p>Woensdag 09:00 tot 13:00\n<p>
                 <p>Donderdag 09:00 tot 13:00\n<p>
                 <p>Vrijdag 09:00 tot 17:00\n<p>
                 <p>Zaterdag 09:00 tot 13:00\n<p>
            <p>Zondag 09:00 tot  13:00\n<p>
        </div>
        <div class="map-element">
            <iframe src="https://www.google.com/maps/embed?pb=!1m14!1m8!1m3!1d1368.916872403317!2d3.5256398085689944!3d51.46866689592991!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c4978c1a642823%3A0x851ef8278504a987!2sRoompot%20Camping%20Dishoek!5e0!3m2!1snl!2snl!4v1732892925130!5m2!1snl!2snl" allowfullscreen="" loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
        </div>
            `;
        container.innerHTML += cardTemplate;
    }
}

