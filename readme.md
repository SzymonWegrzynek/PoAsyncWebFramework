# Orator - Aplikacja Webowa i Klient Konsolowy

Aplikacja webowa oraz klient konsolowy, których zadaniem jest:

- Przyjmowanie parametrów i tekstu pisanego.
- Zwracanie pliku z nagraniem lub odtwarzanie wypowiedzianego tekstu.

## Sposób użycia klienta

Klienta można wywołać w następujący sposób:

1. **Wypowiadanie tekstu:**

   ```bash
   orator <tekst_do_wypowiedzenia>
   ```

   Wypowiada tekst po wysłaniu go do serwera i otrzymaniu zsyntezowanej wersji.

2. **Zapis tekstu do pliku:**

   ```bash
   orator <tekst_do_wypowiedzenia> -o <nazwa_pliku>
   ```

   Zapisuje wypowiedziany tekst do pliku.

3. **Odczyt pliku tekstowego i zapis do pliku:**

   ```bash
   orator -s <nazwa_pliku_tekstowego> -o <nazwa_pliku>
   ```

   Odczytuje plik tekstowy i zapisuje go do pliku.

4. **Ustawianie parametrów:**
   ```bash
   orator -<nazwa_parametru> <wartość>
   ```
   Ustawia parametr.

## Endpointy aplikacji webowej

Aplikacja webowa powinna posiadać co najmniej następujące endpointy:

- **POST /to_speech**  
  Przyjmuje tekst i zwraca plik z nagraniem.

- **PATCH /set_parameter**  
  Ustawia parametry dla generowanego tekstu.

## Problemy do rozwiązania

1. Jak przechować na serwerze parametry dla generowanego tekstu.
2. Jak prawidłowo zaimplementować architekturę trójwarstwową zarówno po stronie serwera, jak i klienta.

## Kryteria oceniania

1. **Klient:**

   - Odczytywanie i parsowanie wejścia (argparse) - **2 pkt**.
   - Architektura trójwarstwowa klienta - **2 pkt**.
   - Testy dla klienta - **1 pkt**.

2. **Serwer:**
   - Użycie frameworka asynchronicznego - **2 pkt**.
   - Architektura trójwarstwowa i wstrzykiwanie zależności - **4 pkt**.
   - Testy dla serwera - **4 pkt**.
