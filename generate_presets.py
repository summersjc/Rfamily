#!/usr/bin/env python3
"""
Generate language preset JSON files for Rfamily GEDCOM generator.
Creates culturally appropriate names and locations for 52 languages.
"""

import json
import os

# Language preset data
PRESETS = {
    "albanian": {
        "male_names": ["Alban", "Arben", "Besnik", "Dritan", "Enver", "Fatmir", "Genc", "Ilir", "Kushtrim", "Luan", "Marko", "Petrit", "Sokol", "Taulant", "Viktor"],
        "female_names": ["Agnesa", "Anjeza", "Besa", "Diana", "Eli", "Flutura", "Genta", "Helena", "Ida", "Klara", "Lindita", "Mirabela", "Nora", "Rozafa", "Valentina"],
        "surnames": ["Hoxha", "Shehu", "Kola", "Meta", "Zeneli", "Leka", "Kurti", "Pashaj", "Rama", "Berisha", "Xhafa", "Nano", "Duka", "Gjoka", "Musta"],
        "country": "Albania",
        "language": "Albanian",
        "cities": ["Tirana", "Durrës", "Vlorë", "Shkodër", "Fier", "Korçë", "Elbasan", "Berat"],
    },
    "bulgarian": {
        "male_names": ["Alexander", "Dimitar", "Georgi", "Ivan", "Kamen", "Nikola", "Petar", "Stefan", "Todor", "Vasil", "Vladimir", "Yordan", "Zlatan", "Boris", "Hristo"],
        "female_names": ["Anna", "Diana", "Elena", "Ivana", "Katya", "Maria", "Nadia", "Penka", "Radka", "Sofia", "Tanya", "Valentina", "Yordanka", "Zlatka", "Daniela"],
        "surnames": ["Ivanov", "Petrov", "Dimitrov", "Georgiev", "Nikolov", "Stoyanov", "Vasilev", "Hristov", "Todorov", "Angelov", "Popov", "Kolev", "Yordanov", "Marinov", "Kostov"],
        "country": "Bulgaria",
        "language": "Bulgarian",
        "cities": ["Sofia", "Plovdiv", "Varna", "Burgas", "Ruse", "Stara Zagora", "Pleven", "Sliven"],
    },
    "croatian": {
        "male_names": ["Ivan", "Marko", "Luka", "Petar", "Mateo", "Josip", "Filip", "Ante", "Tomislav", "Stjepan", "Karlo", "Davor", "Nikola", "Zvonimir", "Miljenko"],
        "female_names": ["Ana", "Marija", "Ivana", "Petra", "Lucija", "Maja", "Sara", "Iva", "Katarina", "Magdalena", "Elena", "Matea", "Vesna", "Dubravka", "Nada"],
        "surnames": ["Horvat", "Kovačić", "Babić", "Marić", "Novak", "Jurić", "Knežević", "Matić", "Pavlović", "Tomić", "Popović", "Vuković", "Perić", "Miletić", "Šimić"],
        "country": "Croatia",
        "language": "Croatian",
        "cities": ["Zagreb", "Split", "Rijeka", "Osijek", "Zadar", "Dubrovnik", "Pula", "Slavonski Brod"],
    },
    "czech": {
        "male_names": ["Jan", "Petr", "Josef", "Pavel", "Jaroslav", "Tomáš", "Martin", "Miroslav", "František", "Jiří", "Michal", "Václav", "Jakub", "Karel", "Lukáš"],
        "female_names": ["Marie", "Jana", "Eva", "Anna", "Hana", "Věra", "Lenka", "Petra", "Alena", "Martina", "Jitka", "Lucie", "Kateřina", "Veronika", "Tereza"],
        "surnames": ["Novák", "Svoboda", "Novotný", "Dvořák", "Černý", "Procházka", "Kučera", "Veselý", "Horák", "Němec", "Pokorný", "Marek", "Pospíšil", "Hájek", "Král"],
        "country": "Czech Republic",
        "language": "Czech",
        "cities": ["Prague", "Brno", "Ostrava", "Plzeň", "Liberec", "Olomouc", "České Budějovice", "Hradec Králové"],
    },
    "danish": {
        "male_names": ["Anders", "Christian", "Erik", "Frederik", "Hans", "Jens", "Kasper", "Lars", "Mads", "Mikkel", "Niels", "Peter", "Rasmus", "Søren", "Thomas"],
        "female_names": ["Anna", "Camilla", "Emma", "Frederikke", "Ida", "Julie", "Karen", "Laura", "Line", "Mette", "Nina", "Rikke", "Sofie", "Trine", "Vibeke"],
        "surnames": ["Jensen", "Nielsen", "Hansen", "Pedersen", "Andersen", "Christensen", "Larsen", "Sørensen", "Rasmussen", "Jørgensen", "Petersen", "Madsen", "Kristensen", "Olsen", "Thomsen"],
        "country": "Denmark",
        "language": "Danish",
        "cities": ["Copenhagen", "Aarhus", "Odense", "Aalborg", "Esbjerg", "Randers", "Kolding", "Horsens"],
    },
    "dutch": {
        "male_names": ["Jan", "Pieter", "Willem", "Hendrik", "Johannes", "Gerrit", "Cornelis", "Dirk", "Adriaan", "Martijn", "Bas", "Lars", "Thijs", "Sander", "Ruud"],
        "female_names": ["Anna", "Maria", "Elisabeth", "Catharina", "Johanna", "Margaretha", "Sophia", "Emma", "Lisa", "Anne", "Sophie", "Eva", "Julia", "Laura", "Nina"],
        "surnames": ["De Jong", "Jansen", "De Vries", "Van den Berg", "Van Dijk", "Bakker", "Janssen", "Visser", "Smit", "Meijer", "De Boer", "Mulder", "De Groot", "Bos", "Vos"],
        "country": "Netherlands",
        "language": "Dutch",
        "cities": ["Amsterdam", "Rotterdam", "The Hague", "Utrecht", "Eindhoven", "Groningen", "Tilburg", "Almere"],
    },
    "estonian": {
        "male_names": ["Jaan", "Peeter", "Mart", "Andres", "Meelis", "Toomas", "Priit", "Raivo", "Urmas", "Ants", "Kalev", "Rein", "Tanel", "Marko", "Kaarel"],
        "female_names": ["Katrin", "Liisa", "Mari", "Anne", "Piret", "Kristiina", "Kadri", "Kaire", "Merle", "Tiina", "Helena", "Sirje", "Margit", "Annika", "Maris"],
        "surnames": ["Tamm", "Sepp", "Saar", "Mägi", "Kask", "Kuusk", "Kukk", "Rebane", "Ilves", "Pärn", "Teder", "Uusmaa", "Lepp", "Org", "Raud"],
        "country": "Estonia",
        "language": "Estonian",
        "cities": ["Tallinn", "Tartu", "Narva", "Pärnu", "Kohtla-Järve", "Viljandi", "Rakvere", "Maardu"],
    },
    "finnish": {
        "male_names": ["Juhani", "Johannes", "Olavi", "Mikael", "Antero", "Tapani", "Jari", "Matti", "Marko", "Jukka", "Pekka", "Timo", "Antti", "Ville", "Oskari"],
        "female_names": ["Maria", "Helena", "Johanna", "Anneli", "Kaarina", "Liisa", "Marjatta", "Annikki", "Riitta", "Sanna", "Laura", "Katariina", "Emilia", "Sofia", "Aino"],
        "surnames": ["Korhonen", "Virtanen", "Mäkinen", "Nieminen", "Mäkelä", "Hämäläinen", "Laine", "Heikkinen", "Koskinen", "Järvinen", "Lehtonen", "Lehtinen", "Saarinen", "Salminen", "Heinonen"],
        "country": "Finland",
        "language": "Finnish",
        "cities": ["Helsinki", "Espoo", "Tampere", "Vantaa", "Oulu", "Turku", "Jyväskylä", "Lahti"],
    },
    "german": {
        "male_names": ["Michael", "Thomas", "Andreas", "Peter", "Klaus", "Wolfgang", "Jürgen", "Hans", "Christian", "Stefan", "Markus", "Martin", "Frank", "Matthias", "Heinrich"],
        "female_names": ["Maria", "Anna", "Elisabeth", "Ursula", "Monika", "Barbara", "Petra", "Sabine", "Andrea", "Claudia", "Katrin", "Julia", "Sophie", "Emma", "Lena"],
        "surnames": ["Müller", "Schmidt", "Schneider", "Fischer", "Weber", "Meyer", "Wagner", "Becker", "Schulz", "Hoffmann", "Schäfer", "Koch", "Bauer", "Richter", "Klein"],
        "country": "Germany",
        "language": "German",
        "cities": ["Berlin", "Hamburg", "Munich", "Cologne", "Frankfurt", "Stuttgart", "Düsseldorf", "Leipzig"],
    },
    "greek": {
        "male_names": ["Georgios", "Ioannis", "Konstantinos", "Dimitrios", "Nikolaos", "Panagiotis", "Vasileios", "Christos", "Athanasios", "Michail", "Alexandros", "Andreas", "Antonios", "Theodoros", "Spyridon"],
        "female_names": ["Maria", "Eleni", "Aikaterini", "Vasiliki", "Sofia", "Angeliki", "Georgia", "Dimitra", "Paraskevi", "Chrysoula", "Anastasia", "Ioanna", "Konstantina", "Athina", "Evangelia"],
        "surnames": ["Papadopoulos", "Papageorgiou", "Papadakis", "Kouris", "Nikolaou", "Georgiou", "Dimitriou", "Vasileiou", "Konstantinou", "Christodoulou", "Petrou", "Ioannou", "Athanasiou", "Michailidis", "Alexandrou"],
        "country": "Greece",
        "language": "Greek",
        "cities": ["Athens", "Thessaloniki", "Patras", "Heraklion", "Larissa", "Volos", "Rhodes", "Ioannina"],
    },
    "hungarian": {
        "male_names": ["László", "István", "József", "János", "Zoltán", "Péter", "Sándor", "Gábor", "Ferenc", "Attila", "András", "Tamás", "Mihály", "Balázs", "Dávid"],
        "female_names": ["Mária", "Erzsébet", "Anna", "Katalin", "Ilona", "Éva", "Judit", "Ágnes", "Margit", "Zsuzsanna", "Andrea", "Krisztina", "Mónika", "Eszter", "Viktória"],
        "surnames": ["Nagy", "Kovács", "Tóth", "Szabó", "Horváth", "Varga", "Kiss", "Molnár", "Németh", "Farkas", "Balogh", "Papp", "Takács", "Juhász", "Lakatos"],
        "country": "Hungary",
        "language": "Hungarian",
        "cities": ["Budapest", "Debrecen", "Szeged", "Miskolc", "Pécs", "Győr", "Nyíregyháza", "Kecskemét"],
    },
    "latvian": {
        "male_names": ["Jānis", "Pēteris", "Andrejs", "Aleksandrs", "Mārtiņš", "Uldis", "Juris", "Valdis", "Guntis", "Aigars", "Artūrs", "Edgars", "Kārlis", "Roberts", "Rihards"],
        "female_names": ["Anna", "Marija", "Elizabete", "Ilze", "Inese", "Ineta", "Liga", "Sanita", "Ieva", "Kristīne", "Laura", "Madara", "Agnese", "Inga", "Liene"],
        "surnames": ["Bērziņš", "Kalniņš", "Liepiņš", "Ozoliņš", "Krūmiņš", "Ozols", "Jansons", "Kļaviņš", "Zariņš", "Balodis", "Briede", "Eglītis", "Feldmanis", "Grīnbergs", "Krastiņš"],
        "country": "Latvia",
        "language": "Latvian",
        "cities": ["Riga", "Daugavpils", "Liepāja", "Jelgava", "Jūrmala", "Ventspils", "Rēzekne", "Valmiera"],
    },
    "lithuanian": {
        "male_names": ["Jonas", "Antanas", "Petras", "Juozas", "Kazys", "Vytautas", "Stasys", "Algirdas", "Mindaugas", "Darius", "Artūras", "Andrius", "Saulius", "Kęstutis", "Valdas"],
        "female_names": ["Marija", "Ona", "Elena", "Aldona", "Birutė", "Janina", "Nijolė", "Regina", "Danutė", "Irena", "Laima", "Rūta", "Aušra", "Giedrė", "Rasa"],
        "surnames": ["Kazlauskas", "Petrauskas", "Jankauskas", "Stankevičius", "Vasiliauskas", "Žukauskas", "Butkus", "Urbonas", "Navickas", "Paulauskas", "Balčiūnas", "Sakalauskas", "Adamonis", "Bartkus", "Grigas"],
        "country": "Lithuania",
        "language": "Lithuanian",
        "cities": ["Vilnius", "Kaunas", "Klaipėda", "Šiauliai", "Panevėžys", "Alytus", "Marijampolė", "Mažeikiai"],
    },
    "macedonian": {
        "male_names": ["Aleksandar", "Gjorgi", "Dimitri", "Petar", "Nikola", "Stefan", "Marko", "Dragan", "Bojan", "Dejan", "Zoran", "Goran", "Igor", "Vlade", "Kire"],
        "female_names": ["Elena", "Maja", "Ana", "Marija", "Ivana", "Katerina", "Biljana", "Vesna", "Natasha", "Gordana", "Daniela", "Jasmina", "Lidija", "Sonja", "Violeta"],
        "surnames": ["Nikolovski", "Petrov", "Stojanovski", "Trajkovski", "Dimitrovski", "Georgievski", "Ivanovska", "Stefanovski", "Angelovski", "Gjorgievski", "Ilievski", "Stojkovski", "Zdravkovski", "Mitrevski", "Ristevski"],
        "country": "North Macedonia",
        "language": "Macedonian",
        "cities": ["Skopje", "Bitola", "Kumanovo", "Prilep", "Tetovo", "Veles", "Ohrid", "Gostivar"],
    },
    "norwegian": {
        "male_names": ["Lars", "Per", "Ole", "Jan", "Bjørn", "Knut", "Erik", "Hans", "Svein", "Arne", "Tor", "Geir", "Morten", "Anders", "Thomas"],
        "female_names": ["Anne", "Ingrid", "Kari", "Marit", "Kristin", "Hilde", "Lise", "Inger", "Grete", "Randi", "Liv", "Solveig", "Berit", "Tone", "Nina"],
        "surnames": ["Hansen", "Johansen", "Olsen", "Larsen", "Andersen", "Pedersen", "Nilsen", "Kristiansen", "Jensen", "Karlsen", "Johnsen", "Pettersen", "Eriksen", "Berg", "Haugen"],
        "country": "Norway",
        "language": "Norwegian",
        "cities": ["Oslo", "Bergen", "Trondheim", "Stavanger", "Drammen", "Fredrikstad", "Kristiansand", "Tromsø"],
    },
    "polish": {
        "male_names": ["Jan", "Andrzej", "Piotr", "Krzysztof", "Stanisław", "Tomasz", "Paweł", "Józef", "Marcin", "Marek", "Michał", "Grzegorz", "Jerzy", "Tadeusz", "Adam"],
        "female_names": ["Maria", "Anna", "Katarzyna", "Małgorzata", "Agnieszka", "Barbara", "Ewa", "Elżbieta", "Zofia", "Krystyna", "Jadwiga", "Teresa", "Joanna", "Magdalena", "Monika"],
        "surnames": ["Nowak", "Kowalski", "Wiśniewski", "Wójcik", "Kowalczyk", "Kamiński", "Lewandowski", "Zieliński", "Szymański", "Woźniak", "Dąbrowski", "Kozłowski", "Jankowski", "Mazur", "Kwiatkowski"],
        "country": "Poland",
        "language": "Polish",
        "cities": ["Warsaw", "Kraków", "Łódź", "Wrocław", "Poznań", "Gdańsk", "Szczecin", "Bydgoszcz"],
    },
    "portuguese": {
        "male_names": ["José", "João", "António", "Manuel", "Francisco", "Pedro", "Carlos", "Paulo", "Miguel", "Fernando", "Rui", "Luís", "Nuno", "Ricardo", "Tiago"],
        "female_names": ["Maria", "Ana", "Manuela", "Francisca", "Paula", "Carla", "Sandra", "Teresa", "Catarina", "Isabel", "Beatriz", "Mariana", "Sofia", "Joana", "Rita"],
        "surnames": ["Silva", "Santos", "Ferreira", "Pereira", "Oliveira", "Costa", "Rodrigues", "Martins", "Jesus", "Sousa", "Fernandes", "Gonçalves", "Gomes", "Lopes", "Marques"],
        "country": "Portugal",
        "language": "Portuguese",
        "cities": ["Lisbon", "Porto", "Amadora", "Braga", "Setúbal", "Coimbra", "Funchal", "Almada"],
    },
    "romanian": {
        "male_names": ["Ion", "Gheorghe", "Nicolae", "Vasile", "Constantin", "Dumitru", "Stefan", "Marin", "Petre", "Florin", "Adrian", "Mihai", "Cristian", "Daniel", "Andrei"],
        "female_names": ["Maria", "Elena", "Ana", "Ioana", "Gheorghita", "Ecaterina", "Florentina", "Mihaela", "Daniela", "Cristina", "Gabriela", "Adriana", "Andreea", "Alexandra", "Raluca"],
        "surnames": ["Popescu", "Ionescu", "Popa", "Constantin", "Dumitrescu", "Stan", "Stoica", "Gheorghe", "Munteanu", "Barbu", "Nistor", "Florea", "Diaconu", "Stanciu", "Moldovan"],
        "country": "Romania",
        "language": "Romanian",
        "cities": ["Bucharest", "Cluj-Napoca", "Timișoara", "Iași", "Constanța", "Craiova", "Brașov", "Galați"],
    },
    "russian": {
        "male_names": ["Alexander", "Dmitry", "Ivan", "Sergey", "Andrey", "Alexey", "Vladimir", "Mikhail", "Nikolay", "Pavel", "Yuri", "Viktor", "Oleg", "Anton", "Denis"],
        "female_names": ["Maria", "Anna", "Elena", "Olga", "Tatyana", "Irina", "Natalia", "Ekaterina", "Svetlana", "Yulia", "Galina", "Ludmila", "Anastasia", "Victoria", "Daria"],
        "surnames": ["Ivanov", "Smirnov", "Kuznetsov", "Popov", "Sokolov", "Lebedev", "Kozlov", "Novikov", "Morozov", "Petrov", "Volkov", "Solovyov", "Vasilyev", "Zaytsev", "Pavlov"],
        "country": "Russia",
        "language": "Russian",
        "cities": ["Moscow", "Saint Petersburg", "Novosibirsk", "Yekaterinburg", "Kazan", "Nizhny Novgorod", "Chelyabinsk", "Samara"],
    },
    "serbian": {
        "male_names": ["Nikola", "Marko", "Stefan", "Lazar", "Nemanja", "Aleksandar", "Petar", "Milan", "Novak", "Jovan", "Dusan", "Zoran", "Dejan", "Dragan", "Igor"],
        "female_names": ["Milica", "Jovana", "Ana", "Marija", "Teodora", "Jelena", "Nina", "Sara", "Sofija", "Katarina", "Ivana", "Maja", "Tamara", "Natasa", "Vesna"],
        "surnames": ["Jovanović", "Petrović", "Nikolić", "Marković", "Đorđević", "Stojanović", "Ilić", "Stanković", "Pavlović", "Milošević", "Dimitrijević", "Simić", "Đukić", "Kostić", "Todorović"],
        "country": "Serbia",
        "language": "Serbian",
        "cities": ["Belgrade", "Novi Sad", "Niš", "Kragujevac", "Subotica", "Zrenjanin", "Pančevo", "Čačak"],
    },
    "slovak": {
        "male_names": ["Ján", "Peter", "Jozef", "Ján", "Miroslav", "Martin", "Marián", "Andrej", "Pavol", "Tomáš", "Juraj", "Milan", "Ladislav", "Michal", "Dušan"],
        "female_names": ["Mária", "Anna", "Zuzana", "Katarína", "Alena", "Martina", "Jana", "Eva", "Lenka", "Petra", "Lucia", "Monika", "Andrea", "Daniela", "Veronika"],
        "surnames": ["Varga", "Tóth", "Nagy", "Horváth", "Kovács", "Szabó", "Molnár", "Baláž", "Gašpar", "Kováč", "Balogh", "Németh", "Papp", "Fekete", "Simon"],
        "country": "Slovakia",
        "language": "Slovak",
        "cities": ["Bratislava", "Košice", "Prešov", "Žilina", "Banská Bystrica", "Nitra", "Trnava", "Martin"],
    },
    "slovenian": {
        "male_names": ["Franc", "Janez", "Ivan", "Anton", "Marko", "Andrej", "Jožef", "Matej", "Luka", "Peter", "Aleksander", "Boštjan", "Gregor", "Mitja", "Tomaž"],
        "female_names": ["Marija", "Ana", "Maja", "Irena", "Nataša", "Mojca", "Andreja", "Barbara", "Katja", "Tanja", "Nina", "Eva", "Petra", "Špela", "Anja"],
        "surnames": ["Novak", "Horvat", "Krajnc", "Kovačič", "Zupančič", "Potočnik", "Mlakar", "Kos", "Vidmar", "Golob", "Kolar", "Hribar", "Kastelic", "Štefan", "Turk"],
        "country": "Slovenia",
        "language": "Slovenian",
        "cities": ["Ljubljana", "Maribor", "Celje", "Kranj", "Velenje", "Koper", "Novo Mesto", "Ptuj"],
    },
    "swedish": {
        "male_names": ["Erik", "Lars", "Karl", "Anders", "Per", "Johan", "Nils", "Sven", "Mikael", "Andreas", "Magnus", "Gustav", "Oskar", "Fredrik", "Daniel"],
        "female_names": ["Anna", "Eva", "Maria", "Karin", "Kristina", "Birgitta", "Elisabeth", "Ingrid", "Margareta", "Linnea", "Emma", "Sofia", "Elin", "Hanna", "Sara"],
        "surnames": ["Andersson", "Johansson", "Karlsson", "Nilsson", "Eriksson", "Larsson", "Olsson", "Persson", "Svensson", "Gustafsson", "Pettersson", "Jonsson", "Jansson", "Hansson", "Bengtsson"],
        "country": "Sweden",
        "language": "Swedish",
        "cities": ["Stockholm", "Gothenburg", "Malmö", "Uppsala", "Västerås", "Örebro", "Linköping", "Helsingborg"],
    },
    "turkish": {
        "male_names": ["Mehmet", "Mustafa", "Ahmet", "Ali", "Hüseyin", "Hasan", "İbrahim", "İsmail", "Yusuf", "Ömer", "Abdullah", "Murat", "Emre", "Burak", "Kemal"],
        "female_names": ["Fatma", "Ayşe", "Emine", "Hatice", "Zeynep", "Elif", "Meryem", "Sultanа", "Esra", "Büşra", "Selin", "Derya", "Aslı", "Özge", "Burcu"],
        "surnames": ["Yılmaz", "Kaya", "Demir", "Şahin", "Çelik", "Yıldız", "Yıldırım", "Öztürk", "Aydın", "Özdemir", "Arslan", "Doğan", "Kılıç", "Aslan", "Çetin"],
        "country": "Turkey",
        "language": "Turkish",
        "cities": ["Istanbul", "Ankara", "Izmir", "Bursa", "Adana", "Gaziantep", "Konya", "Antalya"],
    },
    "ukrainian": {
        "male_names": ["Олександр", "Андрій", "Сергій", "Володимир", "Микола", "Віталій", "Олег", "Іван", "Дмитро", "Василь", "Богдан", "Юрій", "Михайло", "Петро", "Павло"],
        "female_names": ["Олена", "Наталія", "Тетяна", "Ірина", "Ольга", "Марія", "Катерина", "Людмила", "Валентина", "Галина", "Анна", "Світлана", "Віра", "Оксана", "Юлія"],
        "surnames": ["Мельник", "Шевченко", "Бойко", "Коваленко", "Бондаренко", "Ткаченко", "Кравченко", "Ковальчук", "Олійник", "Шевчук", "Клименко", "Лисенко", "Полищук", "Руденко", "Савченко"],
        "country": "Ukraine",
        "language": "Ukrainian",
        "cities": ["Kyiv", "Kharkiv", "Odesa", "Dnipro", "Donetsk", "Zaporizhzhia", "Lviv", "Kryvyi Rih"],
    },
    "arabic": {
        "male_names": ["محمد", "أحمد", "علي", "حسن", "حسين", "عمر", "يوسف", "إبراهيم", "خالد", "عبدالله", "سعيد", "محمود", "طارق", "كريم", "ياسر"],
        "female_names": ["فاطمة", "عائشة", "مريم", "زينب", "خديجة", "سارة", "نور", "ليلى", "هدى", "أميرة", "سلمى", "ياسمين", "دينا", "رنا", "نادية"],
        "surnames": ["العلي", "المحمد", "الأحمد", "الحسن", "الخطيب", "السيد", "عباس", "حمدان", "منصور", "قاسم", "الحمد", "صالح", "موسى", "عثمان", "رشيد"],
        "country": "Saudi Arabia",
        "language": "Arabic",
        "cities": ["Riyadh", "Jeddah", "Mecca", "Medina", "Dammam", "Tabuk", "Buraidah", "Khobar"],
    },
    "armenian": {
        "male_names": ["Արմեն", "Գարեգին", "Գևորգ", "Վահագն", "Տիգրան", "Արարատ", "Սամվել", "Արթուր", "Դավիթ", "Արեգ", "Հայկ", "Սարգիս", "Վարդան", "Մհեր", "Անդրանիկ"],
        "female_names": ["Անի", "Մարիամ", "Գայանե", "Սիրանույշ", "Լուսինե", "Նարինե", "Արփինե", "Գոհար", "Հասմիկ", "Ռուզաննա", "Մարինե", "Սոնա", "Նունե", "Լիլիթ", "Արևիկ"],
        "surnames": ["Հարությունյան", "Գրիգորյան", "Սարգսյան", "Պետրոսյան", "Ավագյան", "Մկրտչյան", "Սահակյան", "Գևորգյան", "Խաչատրյան", "Մանուկյան", "Ղազարյան", "Բարսեղյան", "Մարտիրոսյան", "Ասատրյան", "Դավթյան"],
        "country": "Armenia",
        "language": "Armenian",
        "cities": ["Yerevan", "Gyumri", "Vanadzor", "Vagharshapat", "Hrazdan", "Abovyan", "Kapan", "Armavir"],
    },
    "chinese": {
        "male_names": ["偉", "明", "建國", "建華", "強", "軍", "勇", "傑", "磊", "濤", "鵬", "龍", "浩", "宇", "凱"],
        "female_names": ["秀英", "秀蘭", "麗", "靜", "敏", "玉蘭", "芳", "娟", "麗娜", "雪", "燕", "婷", "莉", "穎", "嫻"],
        "surnames": ["王", "李", "張", "劉", "陳", "楊", "黃", "趙", "吳", "周", "徐", "孫", "馬", "朱", "胡"],
        "country": "Taiwan",
        "language": "Chinese (Traditional)",
        "cities": ["Taipei", "Kaohsiung", "Taichung", "Tainan", "Hsinchu", "Keelung", "Chiayi", "Changhua"],
    },
    "japanese": {
        "male_names": ["太郎", "次郎", "健", "勇", "誠", "浩", "隆", "学", "博", "修", "哲", "昭", "正", "武", "幸"],
        "female_names": ["花子", "美子", "幸子", "洋子", "恵子", "真理子", "由美", "陽子", "明美", "裕子", "直子", "智子", "加奈子", "里奈", "愛"],
        "surnames": ["佐藤", "鈴木", "高橋", "田中", "伊藤", "渡辺", "山本", "中村", "小林", "加藤", "吉田", "山田", "佐々木", "山口", "松本"],
        "country": "Japan",
        "language": "Japanese",
        "cities": ["Tokyo", "Osaka", "Yokohama", "Nagoya", "Sapporo", "Kobe", "Kyoto", "Fukuoka"],
    },
    "korean": {
        "male_names": ["민준", "서준", "도윤", "예준", "시우", "주원", "하준", "지호", "준서", "건우", "현우", "우진", "선우", "연우", "유준"],
        "female_names": ["서연", "서윤", "지우", "서현", "민서", "하은", "하윤", "윤서", "지유", "채원", "지민", "수아", "다은", "예은", "소윤"],
        "surnames": ["김", "이", "박", "최", "정", "강", "조", "윤", "장", "임", "한", "오", "서", "신", "권"],
        "country": "South Korea",
        "language": "Korean",
        "cities": ["Seoul", "Busan", "Incheon", "Daegu", "Daejeon", "Gwangju", "Suwon", "Ulsan"],
    },
    "khmer": {
        "male_names": ["សុខា", "វឌ្ឍនា", "សុវណ្ណ", "ចន្ទ្រា", "រតនៈ", "វិចិត្រ", "ពិសិដ្ឋ", "ស៊ុន", "សំណាង", "ណារិទ្ធ", "វិបុល", "សុភាព", "ស្រីពៅ", "កំពូល", "វឌ្ឍនា"],
        "female_names": ["សុផាត់", "សុផត្រា", "សុវណ្ណី", "ចន្ថា", "ពេជ្រ", "មករា", "សិរីមន", "កញ្ញា", "រស្មី", "ពេជ្រា", "សុភា", "សុគន្ធា", "ណារី", "រដ្ឋា", "វីរៈ"],
        "surnames": ["ហេង", "គឹម", "លី", "ចាន់", "ម៉ៅ", "សុខ", "ទាវ", "ផុន", "ហូ", "នាង", "គង់", "ឈិន", "សួន", "រិទ្ធ", "ជា"],
        "country": "Cambodia",
        "language": "Khmer",
        "cities": ["Phnom Penh", "Siem Reap", "Battambang", "Sihanoukville", "Kampong Cham", "Kampot", "Pursat", "Takeo"],
    },
    "mongolian": {
        "male_names": ["Бат", "Болд", "Дорж", "Эрдэнэ", "Төмөр", "Сүх", "Ганбат", "Мөнх", "Алтан", "Өсөх", "Баяр", "Түвшин", "Цэнд", "Отгон", "Мөнхбат"],
        "female_names": ["Сарнай", "Алтанцэцэг", "Сайнцэцэг", "Долгор", "Энхцэцэг", "Оюунцэцэг", "Мөнхцэцэг", "Номин", "Ариунаа", "Энхтуяа", "Болортуяа", "Батцэцэг", "Одонцэцэг", "Цэцэг", "Үйлс"],
        "surnames": ["Батбаяр", "Энхбаяр", "Доржийн", "Дашийн", "Цэндийн", "Түвшинбаяр", "Баатар", "Сүхбаатар", "Мөнхбаяр", "Алтангэрэл", "Эрдэнэбаяр", "Ганбаяр", "Болдбаяр", "Отгонбаяар", "Нямсүрэн"],
        "country": "Mongolia",
        "language": "Mongolian",
        "cities": ["Ulaanbaatar", "Erdenet", "Darkhan", "Choibalsan", "Mörön", "Khovd", "Ölgii", "Ulaangom"],
    },
    "thai": {
        "male_names": ["สมชาย", "สมศักดิ์", "สมพงษ์", "สมหมาย", "สมบัติ", "ประสิทธิ์", "วิชัย", "สุรศักดิ์", "วีระ", "ธนา", "ชัยยา", "อนุชา", "ธนพล", "กิตติ", "พงษ์ศักดิ์"],
        "female_names": ["สมหญิง", "สมศรี", "นิตยา", "วรรณา", "สุดา", "อัมพร", "พิมพ์", "วิไล", "สุภาพ", "นภา", "ประภา", "มาลี", "สุวรรณ", "ปราณี", "จันทร์"],
        "surnames": ["ชัยวงศ์", "สุขสวัสดิ์", "เจริญสุข", "วงศ์สวัสดิ์", "ธนาวัฒน์", "ศรีสุข", "รุ่งเรือง", "พัฒนาสิน", "สิริมงคล", "เกษมสุข", "ปราณีต", "วงศ์ชัย", "เจริญพร", "สุขเจริญ", "บัวทอง"],
        "country": "Thailand",
        "language": "Thai",
        "cities": ["Bangkok", "Nonthaburi", "Chiang Mai", "Nakhon Ratchasima", "Phuket", "Khon Kaen", "Hat Yai", "Pak Kret"],
    },
    "vietnamese": {
        "male_names": ["Anh", "Dũng", "Hùng", "Tuấn", "Minh", "Phong", "Quân", "Thành", "Hải", "Long", "Bảo", "Khoa", "Nam", "Đức", "Hoàng"],
        "female_names": ["Lan", "Hương", "Mai", "Hoa", "Linh", "Nga", "Thu", "Hằng", "Trang", "Thảo", "Phương", "Nhung", "Vy", "Giang", "Tâm"],
        "surnames": ["Nguyễn", "Trần", "Lê", "Phạm", "Hoàng", "Phan", "Vũ", "Đặng", "Bùi", "Đỗ", "Hồ", "Ngô", "Dương", "Lý", "Võ"],
        "country": "Vietnam",
        "language": "Vietnamese",
        "cities": ["Ho Chi Minh City", "Hanoi", "Da Nang", "Haiphong", "Can Tho", "Bien Hoa", "Hue", "Nha Trang"],
    },
    "fijian": {
        "male_names": ["Jone", "Seru", "Meli", "Petero", "Viliame", "Joji", "Semisi", "Tomasi", "Ratu", "Aisea", "Manasa", "Mosese", "Nemani", "Paula", "Sireli"],
        "female_names": ["Ana", "Maria", "Mere", "Salote", "Vasiti", "Luisa", "Sisilia", "Sera", "Lanieta", "Mereoni", "Nanise", "Rusila", "Titilia", "Adi", "Makereta"],
        "surnames": ["Nailatikau", "Rabuka", "Naiqama", "Tuisova", "Radradra", "Kunavore", "Rokoduru", "Serevi", "Nakarawa", "Volavola", "Goneva", "Vunibaka", "Nadruku", "Kolinisau", "Nadolo"],
        "country": "Fiji",
        "language": "Fijian",
        "cities": ["Suva", "Nadi", "Lautoka", "Labasa", "Nasinu", "Ba", "Nausori", "Sigatoka"],
    },
    "malagasy": {
        "male_names": ["Rakoto", "Rabe", "Rasoa", "Andry", "Hery", "Jean", "Tsiry", "Njaka", "Toky", "Faly", "Miora", "Hasina", "Koto", "Nivo", "Lanto"],
        "female_names": ["Noro", "Rina", "Hanta", "Fara", "Vola", "Ony", "Solofo", "Lalao", "Tianasoa", "Mialy", "Voahangy", "Fenosoa", "Liva", "Narindra", "Ravaka"],
        "surnames": ["Rakotomanga", "Rasolofo", "Randrianasolo", "Andrianaivo", "Rakotomalala", "Randriamampionona", "Ramaroson", "Razafindrakoto", "Randriamanantena", "Rakotonindrina", "Rabemanantsoa", "Rakotonirina", "Raharimalala", "Andrianjafy", "Rasoamanarivo"],
        "country": "Madagascar",
        "language": "Malagasy",
        "cities": ["Antananarivo", "Toamasina", "Antsirabe", "Fianarantsoa", "Mahajanga", "Toliara", "Antsiranana", "Ambatondrazaka"],
    },
    "malay": {
        "male_names": ["Ahmad", "Muhammad", "Ali", "Hassan", "Hussein", "Ibrahim", "Ismail", "Omar", "Yusuf", "Abdullah", "Kamal", "Aziz", "Rashid", "Mahmud", "Hadi"],
        "female_names": ["Fatimah", "Aishah", "Khadijah", "Aminah", "Zainab", "Maryam", "Safiyyah", "Hafsah", "Ummu", "Noor", "Siti", "Nur", "Farah", "Aisyah", "Hidayah"],
        "surnames": ["Abdullah", "Ahmad", "Hassan", "Ibrahim", "Ismail", "Omar", "Rahman", "Ali", "Yusof", "Mohamed", "Hamid", "Mahmud", "Salleh", "Osman", "Sulaiman"],
        "country": "Malaysia",
        "language": "Malay",
        "cities": ["Kuala Lumpur", "George Town", "Ipoh", "Shah Alam", "Petaling Jaya", "Johor Bahru", "Malacca City", "Kuching"],
    },
    "samoan": {
        "male_names": ["Tuilaepa", "Malietoa", "Tupua", "Faumuina", "Mulitalo", "Tofaeono", "Muagututagata", "Tuila'epa", "Seiuli", "Fata", "Leilua", "Tagaloa", "Tapumanaia", "Fiame", "Leota"],
        "female_names": ["Sina", "Leilani", "Tigilau", "Taua", "Moana", "Siliniu", "Fonotoe", "Mareta", "Atalina", "Peseta", "Miriama", "Sala", "Taimalelagi", "Peseta", "Fatumanava"],
        "surnames": ["Tuilaepa", "Sailele", "Malielegaoi", "Tuimalealiifano", "Eti", "Muagututagata", "Leota", "Nafo", "Tupa'i", "Tagaloa", "Faumuina", "Tuiloma", "Fepulea'i", "Rimoni", "Va'ai"],
        "country": "Samoa",
        "language": "Samoan",
        "cities": ["Apia", "Vaitele", "Faleula", "Siusega", "Malie", "Vaiusu", "Afega", "Mulifanua"],
    },
    "tongan": {
        "male_names": ["Sione", "Pita", "Paula", "Tevita", "Viliam", "Siaosi", "Semisi", "Tomasi", "Mosese", "Pasikala", "Uili", "Tevita", "Konisi", "Manu", "Nau"],
        "female_names": ["Ana", "Mele", "Seini", "Ofa", "Vasiti", "Luisa", "Salote", "Malia", "Eseta", "Katalina", "Lesila", "Amelia", "Fanga", "Kaute", "Lupe"],
        "surnames": ["Tupou", "Fusitu'a", "Vaea", "Ma'afu", "Tuku'aho", "Lavulavu", "Moala", "Fonua", "Piutau", "Takulua", "Latu", "Kavaliku", "Pohiva", "Fifita", "Taufa"],
        "country": "Tonga",
        "language": "Tongan",
        "cities": ["Nuku'alofa", "Neiafu", "Haveluloto", "Vaini", "Pangai", "Mu'a", "Nukualofa", "Ohonua"],
    },
    "tagalog": {
        "male_names": ["Juan", "Jose", "Pedro", "Antonio", "Francisco", "Ramon", "Carlos", "Miguel", "Fernando", "Ricardo", "Roberto", "Manuel", "Andres", "Eduardo", "Rafael"],
        "female_names": ["Maria", "Josefa", "Teresa", "Rosa", "Carmen", "Luz", "Esperanza", "Concepcion", "Remedios", "Mercedes", "Dolores", "Gloria", "Fe", "Rosario", "Cristina"],
        "surnames": ["Dela Cruz", "Garcia", "Reyes", "Ramos", "Mendoza", "Santos", "Flores", "Gonzales", "Bautista", "Cruz", "Lopez", "Castillo", "Torres", "Rivera", "Villanueva"],
        "country": "Philippines",
        "language": "Tagalog",
        "cities": ["Manila", "Quezon City", "Caloocan", "Davao", "Cebu City", "Zamboanga", "Taguig", "Antipolo"],
    },
    "swahili": {
        "male_names": ["Hassan", "Ali", "Juma", "Rashid", "Salim", "Hamisi", "Bakari", "Omari", "Seif", "Issa", "Abdallah", "Musa", "Habibu", "Iddi", "Athuman"],
        "female_names": ["Fatuma", "Asha", "Zainab", "Amina", "Halima", "Mwanahamisi", "Subira", "Rukia", "Mariam", "Saada", "Hidaya", "Jamila", "Khadija", "Rehema", "Salma"],
        "surnames": ["Juma", "Hassan", "Mohamed", "Ali", "Rashid", "Salim", "Bakari", "Omari", "Seif", "Hamisi", "Abdallah", "Musa", "Iddi", "Athuman", "Habibu"],
        "country": "Tanzania",
        "language": "Swahili",
        "cities": ["Dar es Salaam", "Mwanza", "Zanzibar City", "Arusha", "Dodoma", "Mbeya", "Morogoro", "Tanga"],
    },
    "haitian": {
        "male_names": ["Jean", "Pierre", "Jacques", "Joseph", "Marc", "Louis", "Paul", "Michel", "André", "Philippe", "Emmanuel", "Claude", "François", "Robert", "Antoine"],
        "female_names": ["Marie", "Anne", "Rose", "Jacqueline", "Claudette", "Josette", "Simone", "Françoise", "Michelle", "Jeanne", "Paulette", "Monique", "Nicole", "Denise", "Yvette"],
        "surnames": ["Jean", "Pierre", "Joseph", "Baptiste", "Louis", "Philogene", "Francois", "Charles", "Michel", "Felix", "Jacques", "Paul", "Antoine", "Emmanuel", "Simon"],
        "country": "Haiti",
        "language": "Haitian Creole",
        "cities": ["Port-au-Prince", "Cap-Haïtien", "Gonaïves", "Les Cayes", "Pétionville", "Jacmel", "Saint-Marc", "Carrefour"],
    },
    "guarani": {
        "male_names": ["José", "Juan", "Carlos", "Luis", "Miguel", "Pedro", "Antonio", "Francisco", "Ramón", "Jorge", "Roberto", "Ricardo", "Fernando", "Andrés", "Raúl"],
        "female_names": ["María", "Rosa", "Carmen", "Ana", "Juana", "Teresa", "Concepción", "Mercedes", "Isabel", "Francisca", "Lucía", "Elena", "Beatriz", "Gloria", "Patricia"],
        "surnames": ["González", "Rodríguez", "López", "Martínez", "Fernández", "García", "Benítez", "Romero", "Acosta", "Medina", "Cabrera", "Vera", "Sosa", "Ayala", "Villalba"],
        "country": "Paraguay",
        "language": "Guarani",
        "cities": ["Asunción", "Ciudad del Este", "San Lorenzo", "Luque", "Capiatá", "Lambaré", "Fernando de la Mora", "Limpio"],
    },
    "cebuano": {
        "male_names": ["Juan", "Pedro", "Jose", "Antonio", "Manuel", "Vicente", "Domingo", "Francisco", "Mariano", "Santiago", "Pablo", "Andres", "Miguel", "Rafael", "Luis"],
        "female_names": ["Maria", "Josefa", "Rosa", "Carmen", "Francisca", "Petra", "Juana", "Trinidad", "Concepcion", "Felicidad", "Dolores", "Mercedes", "Luz", "Esperanza", "Gloria"],
        "surnames": ["Fernandez", "Rodriguez", "Gonzales", "Santos", "Martinez", "Lopez", "Cruz", "Reyes", "Ramos", "Flores", "Mendoza", "Garcia", "Bautista", "Castro", "Domingo"],
        "country": "Philippines",
        "language": "Cebuano",
        "cities": ["Cebu City", "Mandaue", "Lapu-Lapu", "Toledo", "Danao", "Talisay", "Naga", "Carcar"],
    },
    "farsi": {
        "male_names": ["محمد", "علی", "حسن", "حسین", "رضا", "احمد", "مهدی", "امیر", "سعید", "جواد", "مصطفی", "ابراهیم", "اصغر", "اکبر", "ناصر"],
        "female_names": ["فاطمه", "زهرا", "مریم", "زینب", "سکینه", "مهناز", "پروین", "شیرین", "سارا", "نرگس", "نسیم", "لیلا", "الهام", "مینا", "سمیرا"],
        "surnames": ["احمدی", "محمدی", "رضایی", "حسینی", "علیزاده", "صادقی", "مرادی", "کریمی", "رحیمی", "عباسی", "نوری", "موسوی", "اکبری", "قاسمی", "جعفری"],
        "country": "Iran",
        "language": "Farsi",
        "cities": ["Tehran", "Mashhad", "Isfahan", "Karaj", "Shiraz", "Tabriz", "Qom", "Ahvaz"],
    },
}


def create_preset_json(lang_key, data):
    """Create a complete ruleset JSON for a language."""
    preset = {
        "names": {
            "male_given_names": data["male_names"],
            "female_given_names": data["female_names"],
            "surnames": data["surnames"],
            "use_patronymic": False,
            "use_matronymic": False,
            "name_format": "WesternStyle"
        },
        "dates": {
            "birth_year_start": 1700,
            "birth_year_end": 2010,
            "min_marriage_age": 18,
            "max_marriage_age": 45,
            "min_parent_age": 16,
            "max_parent_age": 50,
            "life_expectancy_mean": 75,
            "life_expectancy_stddev": 15,
            "include_death_dates": True
        },
        "locations": {
            "countries": [
                {
                    "name": data["country"],
                    "language": data["language"],
                    "cities": data["cities"],
                    "probability_weight": 1.0
                }
            ],
            "default_country": data["country"]
        },
        "demographics": {
            "sex_ratio": 0.51,
            "twin_rate": 0.032,
            "triplet_rate": 0.001,
            "languages": [data["language"]]
        },
        "relationships": {
            "marriage_probability": 0.85,
            "divorce_probability": 0.40,
            "remarriage_probability": 0.50,
            "children_mean": 2.5,
            "children_stddev": 1.5,
            "min_children": 0,
            "max_children": 12,
            "generate_families": True,
            "multi_generational": True,
            "generations": 4
        },
        "ordinances": {
            "include_lds_ordinances": False,
            "baptism_probability": 0.0,
            "confirmation_probability": 0.0,
            "endowment_probability": 0.0,
            "sealing_to_parents_probability": 0.0,
            "sealing_to_spouse_probability": 0.0,
            "temples": ["SLAKE", "PROVO", "MANTI"]
        }
    }
    
    return preset


def main():
    """Generate all language preset files."""
    presets_dir = "presets"
    
    # Skip existing presets
    existing = {"english", "spanish", "french", "italian", "icelandic", "lds"}
    
    created_count = 0
    for lang_key, data in PRESETS.items():
        if lang_key in existing:
            print(f"Skipping {lang_key} (already exists)")
            continue
            
        filename = os.path.join(presets_dir, f"{lang_key}.json")
        preset = create_preset_json(lang_key, data)
        
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(preset, f, ensure_ascii=False, indent=2)
        
        print(f"Created {filename}")
        created_count += 1
    
    print(f"\nSuccessfully created {created_count} new language presets!")
    print(f"Total presets: {len(existing) + created_count}")


if __name__ == "__main__":
    main()
