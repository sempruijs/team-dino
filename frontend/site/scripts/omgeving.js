const places = [
    {   
        image: "47.jpg",
        alt: "Strand",
        title: "Strand van Dishoek",
        text: "Waai lekker uit of geniet van de zon op het brede zandstrand van Dishoek. Vanaf het strand zie je grote zeeschepen vlak langs de kustlijn varen, een uniek gezicht! Het strand van Dishoek heeft de internationale Blauwe Vlag onderscheiding. Het zwemwater is dus van uitstekende kwaliteit en het zandstrand wordt goed schoongehouden.",
        adress: "Dishoek 1, Dishoek",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9941.737746281782!2d3.514049679591374!3d51.46853954886973!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c497acc703caf5%3A0xd087cd0280d79a45!2sStrand%20Dishoek!5e0!3m2!1snl!2snl!4v1732979455575!5m2!1snl!2snl"
    },

    {
        image: "48.jpg",
        alt: "Vuurtoren",
        title: "Vuurtorens van Kaapduinen",
        text: "Wandel door de duinen van Dishoek langs de twee herkenbare gestreepte vuurtorens van Kaapduinen. Vanaf de duinen heb je prachtig uitzicht over het strand en de Noordzee. Spot jij de schepen in de verte al?",
        adress: "Dishoek 99, Koudekerke",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d5910.5892260894225!2d3.515362144850296!3d51.47474028967733!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c49734b2538667%3A0x2036d028204413c7!2sVuurtoren%20Dishoek%20Kaapduinen%20%E2%80%98t%20Lage%20Licht!5e0!3m2!1snl!2snl!4v1732979338389!5m2!1snl!2snl"
    },

    {
        image: "49.jpg",
        alt: "Middelburg",
        title: "Middelburg",
        text: "Middelburg heeft een rijke geschiedenis die je nog steeds in het huidige straatbeeld terugziet. De vele evenementen, winkels en horecagelegenheden, maken het een levendige stad. Waar ook de Zeeuwen zelf graag gaan winkelen of op een terras neerstrijken. Ontdek de hoofdstad van Zeeland!",
        adress: "Achter De Houttuinen 39, Middelburg",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2483.8889401609504!2d3.6069714000000004!3d51.4969055!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c490d6d63ec551%3A0x1f24e063effef477!2sAchter%20de%20Houttuinen%2039%2C%204331%20NJ%20Middelburg!5e0!3m2!1snl!2snl!4v1732979545931!5m2!1snl!2snl"
    },

    {
        image: "50.jpg",
        alt: "Zeeuws Maritiem MuZEEum",
        title: "Zeeuws Maritiem MuZEEum",
        text: "Zeeland heeft een spannend en avontuurlijk verleden: scheepvaart, visserij, Michiel de Ruyter. Het hoort allemaal bij deze provincie. Kom naar het MuZEEum, midden in het gezellige Vlissingen. En ervaar het roerige Zeeuwse leven. Er is van alles te zien, het hele jaar door. Kinderen kunnen een leuke speurtocht doen en met de gratis audiotour wordt het allemaal nóg spannender.",
        adress: "Nieuwendijk 11, Vlissingen",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9947.647374097092!2d3.565131679568815!3d51.441412848771755!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c499ed1a8989a9%3A0x762dbf0e2da9f7e8!2sNieuwendijk%2011%2C%204381%20BV%20Vlissingen!5e0!3m2!1snl!2snl!4v1732979593794!5m2!1snl!2snl"
    },

    {
        image: "55.jpg",
        alt: "Zeeuws Museum",
        title: "Zeeuws Museum",
        text: "In de indrukwekkende Abdij van Middelburg vind je het Zeeuws Museum. Hier draait alles om de provincie Zeeland en het Zeeuws erfgoed. Geschiedenis en archeologie, natuurhistorie, kunst, mode en nog veel meer. Grote wandtapijten met wereldberoemde zeeslagen uit de 16e eeuw bijvoorbeeld. Of een echte mummie. Beleef de geschiedenis van Zeeland opnieuw.",
        adress: "Abdijplein, Middelburg",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9934.771504559565!2d3.604484179617994!3d51.50050324898561!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c49126da89f2e1%3A0xacbbcb1af4b67196!2sAbdijplein!5e0!3m2!1snl!2snl!4v1732979674346!5m2!1snl!2snl"
    },

    {
        image: "56.jpg",
        alt: "Mini Mundi",
        title: "Mini Mundi",
        text: "Welkom in familiepretpark Mini Mundi. Dwaal door het miniatuurpark en beleef Walcheren in het klein. Even verderop vind je leuke attracties, zoals het schommelschip, stoere skelters en de achtbaan voor kinderen vanaf 4 jaar. In de coole indoorspeeltuin leven kinderen zich lekker uit. De houten klimtuin is hier favoriet. Even uitrusten? Het restaurant heeft lekkere pannenkoeken, frietjes en broodjes.",
        adress: "Podium 35, Middelburg",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9939.205026391955!2d3.6218134796010535!3d51.48016224891177!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c490ed4973c175%3A0xd3ed89ad9ef0bfa2!2sPodium%2035%2C%204337%20WV%20Middelburg!5e0!3m2!1snl!2snl!4v1732979727839!5m2!1snl!2snl"
    },

    {
        image: "57.jpg",
        alt: "GlowGolf",
        title: "GlowGolf",
        text: "Minigolf, maar dan véél leuker. Dat is GlowGolf. Een unieke belevenis die je meegemaakt moet hebben. De overdekte golfbaan wordt verlicht door fluorescerende blacklights. Een beetje donker, een beetje spannend. En superleuk. Tijdens het 18-holes parcours sta je oog in oog met olifanten, tijgers en andere jungledieren. Een heerlijk uitje voor jong en oud.",
        adress: "Podium 21, Middelburg",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d23639.406818435364!2d3.610577014611915!3d51.48043206194754!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c4908d2dc87059%3A0x9bc91727e13c8269!2sPodium%2021%2C%204337%20WV%20Middelburg!5e0!3m2!1snl!2snl!4v1732979781693!5m2!1snl!2snl"
    },

    {
        image: "58.jpg",
        alt: "Polderhuis Westkapelle",
        title: "Polderhuis Westkapelle",
        text: "In het dijk- en oorlogsmuseum wandelt u door middel van 3 permanente exposities door ‘7 eeuwen dijk’. De ontwikkelingen van de kustlijn, de geschiedenis van het dorp en de omgeving zijn de rode draad van het museum. Op de begane grond vindt u een expositie over de oorlog, met centraal de bewoners van Westkapelle en hun ervaringen. Ook voor kinderen is er van alles te ontdekken. Er zijn gratis leerzame speurtochten en puzzels voor kinderen vanaf 7 jaar of om met het hele gezin te doen. Tip: combineer uw museumbezoek met het beklimmen van de vuurtorens van Westkapelle. Bovenop deze torens komen de verhalen uit het museum pas echt tot leven!",
        adress: "Zuidstraat 156, Westkapelle",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9928.958798309608!2d3.425291879640187!3d51.52716324908269!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c4bc4790422865%3A0xb0341de5beb2d49b!2sZuidstraat%20156%2C%204361%20AK%20Westkapelle!5e0!3m2!1snl!2snl!4v1732979822745!5m2!1snl!2snl"
    },

    {
        image: "59.jpg",
        alt: "Watersportcentrum Shotsman",
        title: "Watersportcentrum Shotsman",
        text: "Waterskieën, wakeboarden, monoskiën of lekker ravotten in het aquapark. Beleef een spetterende dag op en in het water. De 820 meter lange waterski-kabelbaan is geschikt voor beginners en gevorderden. Voor de kleintjes is er een mini-kabelskibaan. Groepen kunnen banaanvaren, kajakken en suppen. Opgedroogd? Tijd voor een hapje en een drankje in het restaurant. Lekker alle actiefoto's en filmpjes bekijken.",
        adress: "Campensweg 3, Kamperland",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d4959.624541611309!2d3.646873215060129!3d51.57167492532239!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c493b26bc846c7%3A0x743042af42c57765!2sCampensweg%203%2C%204493%20MN%20Kamperland!5e0!3m2!1snl!2snl!4v1732979866231!5m2!1snl!2snl"
    },

    {
        image: "60.jpg",
        alt: "Movement Sports",
        title: "Movement Sports",
        text: "Lekker relaxed suppen. Elkaar lanceren met water blobben. Spectaculair powerkiten. Glijden over de branding op je skimboard. Outdoor centrum Movement Sports is dé plek voor wie dol is op sport, actie en water. Leuk voor iedereen. Ervaren of totaal onervaren. Superfit of gelegenheidssporter. Doe mee en maak je vakantie bij De Groene Weide compleet.",
        adress: "Sophia Boulevard 29, Kamperland",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9914.792618617488!2d3.710566079694288!3d51.59209524932083!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c48d7ad913b627%3A0x2dcb8fad7741edd7!2sSophia%20Boulevard%2029%2C%204493%20PL%20Kamperland!5e0!3m2!1snl!2snl!4v1732979901593!5m2!1snl!2snl"
    },

    {
        image: "61.jpg",
        alt: "Beach Escape",
        title: "Beach Escape",
        text: "Een escape room, maar dan anders. Lekker in de buitenlucht. Je gaat schatzoeken op het strand. Dat doe je met je team, dus samenwerken is belangrijk. De opdrachten zijn soms pittig, maar met elkaar kom je er wel uit toch? Ondertussen maak je ook nog eens een mooie wandeltocht. Ben je gek op spelletjes en avontuur? Dan is dit het ideale uitje.",
        adress: "Sophia Boulevard 29, Kamperland",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9914.792618617488!2d3.710566079694288!3d51.59209524932083!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c48d7ad913b627%3A0x2dcb8fad7741edd7!2sSophia%20Boulevard%2029%2C%204493%20PL%20Kamperland!5e0!3m2!1snl!2snl!4v1732979901593!5m2!1snl!2snl"
    },

    {
        image: "62.jpg",
        alt: "Stroomtrein Goes-Borssele",
        title: "Stroomtrein Goes-Borssele",
        text: "Het avontuur start in Goes. De glimmende locomotief en de prachtig gerestaureerde rijtuigen staan klaar voor vertrek. Onderweg kom je ogen en oren te kort voor het mooie landschap van de Zak van Zuid-Beveland.",
        adress: "Albert Plesmanweg 23, Goes",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9935.460754097105!2d3.8688657796153483!3d51.49734134897408!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c489cbc933136f%3A0x93464a0f0a5d2e99!2sDe%20Poel%20I%2C%20Albert%20Plesmanweg%2023%2C%204462%20GC%20Goes!5e0!3m2!1snl!2snl!4v1732979947142!5m2!1snl!2snl"
    },

    {
        image: "63.jpg",
        alt: "Klimbos Westerschouwen",
        title: "Klimbos Westerschouwen",
        text: "Je bevindt je hoog boven de grond. Je zit goed vast maar toch is het spannend. Met tokkelbanen, touwladders en wiebelbruggen ga je van boom tot boom. Er zijn zes verschillende uitdagende parcoursen in het klimbos. De ene wat hoger en spannender dan de andere. Dus of je beginner bent of gevorderd maakt niet uit. Ga jij de uitdaging aan en wordt je ook een van de Zeeuwse Helden?",
        adress: "Duinpad 1, Burgh-Haamstede",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9897.52188207113!2d3.6940401797602433!3d51.671178449614004!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c4ed9c40486533%3A0xd66e87b96bdf6f39!2sDuinpad%201%2C%204328%20PK%20Burgh-Haamstede!5e0!3m2!1snl!2snl!4v1732979979365!5m2!1snl!2snl"
    },

    {
        image: "64.jpg",
        alt: "Historisch Museum de Bevelanden",
        title: "Historisch Museum de Bevelanden",
        text: "Je bevindt je hoog boven de grond. Je zit goed vast maar toch is het spannend. Met tokkelbanen, touwladders en wiebelbruggen ga je van boom totMidden in het historische centrum van Goes vind je dit bijzondere museum. In een voormalig klooster en weeshuis. Ontdek hoe de wezen daar leefden. Leer over de schutterij, merklappen en de Zeeuwse klederdrachten. Loop door verdronken land zonder natte voeten te krijgen. Kinderen verkennen het museum met leuke speurtochten. En maak daarna nog een mooie wandeling door het gezellige stadje.",
        adress: "Singelstraat 13, Goes",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9934.285617661739!2d3.879449979619844!3d51.50273214899369!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c489d00fabcc93%3A0x51e573ae68cc15a8!2sSingelstraat%2013%2C%204461%20HZ%20Goes!5e0!3m2!1snl!2snl!4v1732980022275!5m2!1snl!2snl"
    },

    {
        image: "65.jpg",
        alt: "Fietsen in Dishoek",
        title: "Fietsen in Dishoek",
        text: "Heeft u zin om al fietsend de omgeving van Dishoek te verkennen? Via Fietsnetwerk.nl kunt u heel eenvoudig een fietsroute samenstellen die past bij uw persoonlijke voorkeuren. De speciale Fietsnetwerk App geeft u leuke tips voor onderweg: perfecte afstapmomenten, mooie bezienswaardigheden en gezellige restaurants. Aan het einde van de dag komt u na een prachtige fietstocht voldaan weer bij uw vakantieverblijf!",
        adress: "Waar u maar wilt",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d39761.267154287685!2d3.4820405671685104!3d51.475060582134034!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c49792488554ff%3A0xd0b2b6c4e06cb1b0!2sDishoek%2C%204371%20NR%20Koudekerke!5e0!3m2!1snl!2snl!4v1732980058192!5m2!1snl!2snl"
    },

    {
        image: "66.jpg",
        alt: "Reptielenzoo IGUANA",
        title: "Reptielenzoo IGUANA",
        text: "Iguana is een opvangcentrum voor reptielen, amfibieën en insecten. Uitgegroeid tot Europa's grootste overdekte reptielenzoo. Dwaal vanuit de serre langs kikkers, salamanders, slangen, hagedissen, schildpadden en vogelspinnen. Sta oog in oog met een krokodil, vuurbuikpad of python. Iguana is een belevenis die je als dierenliefhebber niet mag missen.",
        adress: "Bellamypark 31, Vlissingen",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9947.545721038345!2d3.5624839795692185!3d51.44187954877344!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c499ec8e982217%3A0x4653211312d23c49!2sBellamypark%2031%2C%204381%20CH%20Vlissingen!5e0!3m2!1snl!2snl!4v1732980090998!5m2!1snl!2snl"
    },

    {
        image: "67.jpg",
        alt: "Zwemparadijs De Parel",
        title: "Zwemparadijs De Parel",
        text: "De Parel is hèt vrijetijdscentrum in Domburg. Met o.a. een binnenbad, buitenbad, peuterbad, 79 meter lange glijbaan, pelikaanduik, snelstromende rivier, borrelbanken en een speel- en zonneweide is het genieten voor het hele gezin. De Parel beschikt daarnaast over een sfeervol saunalandschap. Kom tot rust in een van de 3 saunacabines, de kleuren-kruidenkamer, infrarood sauna, bubbelbaden of de patio met buitenbad. In de sfeervolle saunabar geniet je van een hapje, drankje of een smakelijke, gezonde maaltijd.",
        adress: "Babelweg 2, Domburg",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9921.731170724932!2d3.482575979667789!3d51.56029904920392!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c4be1a87006d3d%3A0x315e4a88ab62d1d0!2sBabelweg%202%2C%204357%20BT%20Domburg!5e0!3m2!1snl!2snl!4v1732980122847!5m2!1snl!2snl"
    },

    {
        image: "68.jpg",
        alt: "Oosterschelde Rondvaart",
        title: "Oosterschelde Rondvaart",
        text: "De Oosterschelde is het grootste Nationaal Park van Nederland. Door het getij verandert het landschap steeds weer. Bij eb stroomt 800 miljard liter water de Oosterschelde uit. Vogels zoeken dan naar voedsel op de drooggevallen zandbanken. Frisia Rondvaarten heeft verschillende boottochten door het mooie gebied. Onderweg zie je bijzondere watervogels. En met een beetje geluk zelfs zeehonden en bruinvissen. Vergeet je verrekijker niet!",
        adress: "Marinahaven Sophia Boulevard, Kamperland",
        place: "https://www.google.com/maps/embed?pb=!1m16!1m12!1m3!1d9914.656080750658!2d3.7066528796948046!3d51.59272079932315!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!2m1!1sMarinahaven%20Sophia%20Boulevard%2C%20Kamperland!5e0!3m2!1snl!2snl!4v1732980189773!5m2!1snl!2snl"
    },

    {
        image: "69.jpg",
        alt: "Berkenhof Tropical Zoo",
        title: "Berkenhof Tropical Zoo",
        text: "Lekker dwalen door de mini-jungle. Kijk, daar ritselt een leguaan door het groen. Durf je een vogelspin vast te houden? In Tropical Zoo beleef je het allemaal. Van dichtbij of van iets verder af. Ga ondergronds in de Fossielenmijn en ontdek mammoeten en sabeltandtijgers. Sta oog in oog met dinosaurussen in de Dino Expo. Spelen, klimmen en klauteren doe je lekker in de indoor Kids jungle en het Buitenspeelbos.",
        adress: "Langeweegje 10A, Kwadendamme",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9948.065850157736!2d3.8793873795672074!3d51.43949154876483!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c487f782bf26d7%3A0xa61d295c284fe70e!2sLangeweegje%2010A%2C%204434%20NE%20Kwadendamme!5e0!3m2!1snl!2snl!4v1732980226570!5m2!1snl!2snl"
    },

    {
        image: "70.jpg",
        alt: "Vlissingen",
        title: "Vlissingen",
        text: "Wandel over de 2 kilometer lange boulevard langs de Noordzee. Pak een filmpje in bioscoop CineCity, bezoek het Zeeuws Maritiem muZEEum of ontdek Vlissingen met de Zonnetrein. Na een gezellige dag in de stad geniet je van een diner bij een van de vele cafés en restaurants langs de boulevard, mét uitzicht op zee.",
        adress: "Vlissingen",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d79550.5812047976!2d3.48519879053951!3d51.45897047397025!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c499eddc5a203d%3A0x88182e73f1865efb!2sVlissingen!5e0!3m2!1snl!2snl!4v1732980254279!5m2!1snl!2snl"
    },

    {
        image: "71.jpg",
        alt: "Terra Maris",
        title: "Terra Maris",
        text: "Terra Maris betekent letterlijk: land van de zee. In het museum én de 2,5 ha grote landschapstuin ontdek je de Zeeuwse natuur. Verschillende zeeaquaria, prachtige maquettes, VR-films, 3D-kijkers, skeletten, archeologische vondsten en opgezette dieren vertellen alles over het ontstaan van het Zeeuwse landschap en de natuur van het deltagebied. Voor een lekkere koffie & thee met zelfgebakken taart of een heerlijke lunch kom je naar het gezellige museumcafé ‘de Orangerie’. En in de museumwinkel is voor jong en oud altijd wel iets leuks te vinden. Kijk ook op de website voor alle extra activiteiten en evenementen! Tip: Als gast van De Groene Weide ontvang je korting op je entreebewijs.",
        adress: "Duinvlietweg 6, Oostkapelle",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9919.67937411931!2d3.509665779675637!3d51.56970294923847!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c495928e5202bb%3A0xf69ee670bd836f0!2sDuinvlietweg%206%2C%204356%20ND%20Oostkapelle!5e0!3m2!1snl!2snl!4v1732980288718!5m2!1snl!2snl"
    },

    {
        image: "72.jpg",
        alt: "Deltapark Neeltje Jans",
        title: "Deltapark Neeltje Jans",
        text: "Deltapark Neeltje Jans is een avontuurlijk dagje uit voor de hele familie. Het park ligt midden in het prachtige Nationaal Park de Oosterschelde. Hier draait alles om zon, zee, natuur, water, wind, cultuur en techniek. Beleef het historische verhaal in de Delta Experience, ga op pad door de Delta Expo en naar de stormvloedkering met een gids, bezoek de zeeleeuwen- en zeehondenpresentatie, sta oog in oog met tropische haaien in het grootste Zeeuwse zoutwater aquarium Bluereef, cool af in de Aquasplash, glij naar benden in de waterglijbaan, waai weg in onze orkaanmachine en rust uit in de parktrein. Kortom er is van alles te doen voor een topdag. Zeehonden en bruinvissen spotten? Ga dan mee met de rondvaartboot Christiaan B!",
        adress: "Faelweg 5, Neeltje Jans, Vrouwenpolder",
        place: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d9904.98178292271!2d3.701467779731754!3d51.63702994948698!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c4ed5993d5d309%3A0x43d9d0e4c1a712f8!2sDelta%20Plaza%2C%20Faelweg%205%2C%204354%20RB%20Vrouwenpolder!5e0!3m2!1snl!2snl!4v1732980321427!5m2!1snl!2snl"
    }
];

function omgeving(){
    return places;
}