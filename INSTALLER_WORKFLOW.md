# Installer Workflow

A Soolin Noctra OS telepítő célja, hogy a felhasználó már a telepítés közben találkozzon Noctrával.

A telepítő nem csak technikai lépések sorozata, hanem első rendszerélmény.

## Telepítő céljai

- egyszerű telepítés
- érthető nyelvezet
- minimál, sötét Noctra vizuál
- kezdő felhasználóknak is biztonságos flow
- fejlesztőknek gyors konfiguráció
- választási lehetőség: CLS vagy Desktop előtér

## Telepítési módok

### CLS / CLI-first mód
Ez az elsődleges fejlesztési cél.

A rendszer terminálközpontúan indul, de minimál grafikus réteget tartalmaz:
- boot screen
- login screen
- terminál vizuális stílus
- GUI appok futtatásához szükséges alap réteg

### Desktop mód
Későbbi kiadás / patch része.

Tartalmazza:
- Noctra Desktop
- Noctra Hub
- dock
- fájlkezelő vizuál
- külön Noctra Helper panel

## Első kapcsolat Noctrával

A telepítőben Noctra röviden bemutatkozhat, de nem tolhatja túl magát.

Példa hangnem:

"Szia.
Noctra vagyok.
A Soolin Noctra rendszerentitása.

Segítek végigmenni a telepítésen.
Te döntesz, én figyelek."

## Telepítő fő lépései

1. Nyelv kiválasztása
2. Billentyűzet kiválasztása
3. Hálózat ellenőrzése
4. Lemez kiválasztása
5. Telepítési mód kiválasztása
   - CLS / CLI-first
   - Desktop
6. Felhasználó létrehozása
7. Alap rendszer telepítése
8. Első indítás előkészítése
9. Reboot

## Fontos UX szabályok

- Ne legyen túl sok szöveg.
- Ne használjon túl technikai nyelvet.
- Mindig jelezze, ha adat törlődik.
- A veszélyes műveleteknél kérjen megerősítést.
- A public Noctra legyen nyugodt, segítőkész, elegáns.
- A developer Noctra csak privát buildben lehet szarkasztikusabb.

## Első boot utáni onboarding

Az első bejelentkezés után Noctra felajánlja az alap programok telepítését.

Választható profilok:
- Fejlesztői környezet
- Gamer környezet
- Streamer környezet
- Minimál rendszer
- Egyedi kiválasztás
- Kihagyás

## Noctra Help jelentése

A `noctra help` nem hagyományos súgó.

Nem előre megírt menürendszer, hanem interaktív segítség a terminálban.

Feladata:
- magyarázat
- parancsjavaslat
- rendszerállapot ellenőrzés
- telepítési segítség
- hibák értelmezése
- felhasználó vezetése

A cél nem az, hogy a user dokumentációt olvasson.
A cél az, hogy beszélni tudjon a rendszerrel.
