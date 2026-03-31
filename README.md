-- --- .-. ... . / -.-. --- -.. . / - .-. .- -. ... .-.. .- - --- .-.

# Morse Code Translator
.- / - .-. .- -. ... .-.. .- - --- .-. / .- .--. .--. / ..-. --- .-. / -.-. --- -. ...- . .-. - .. -. --. / -... . - .-- . . -. / -- --- .-. ... . / -.-. --- -.. . --..-- / - . -..- - --..-- / .- -. -.. / .- ..- -.. .. --- .-.-.-

A translator app for converting between Morse Code, text, and audio.

## Usage
To translate a given string of text into Morse Code, run:

```sh
morse-code-translator --to-morse "Hello, world\!"
# .... . .-.. .-.. --- --..-- / .--- --- .-. .-.. -.. -.-.--

morse-code-translator -m "Hello, world\!"
# .... . .-.. .-.. --- --..-- / .--- --- .-. .-.. -.. -.-.--
```

To translate text from Morse to plain text:
```sh
morse-code-translator --to-text ".... .. --..-- / .. .----. -- / ... .- -- / -- --- .-. ... ."
# HI, I'M SAM MORSE

morse-code-translator -t ".... .. --..-- / .. .----. -- / ... .- -- / -- --- .-. ... ."
# HI, I'M SAM MORSE
```

Alternatively, if you want to translate the contents of an entire file, you can do so providing a valid file path to the `-f` flag like so:

`groceries.txt`
```txt
Fruits:
Apple
Banana
Cherry
Pear
```

```sh
morse-code-translator -m -f groceries.txt
# ..-. .-. ..- .. - ... ---... .-.- .- .--. .--. .-.. . .-.- -... .- -. .- -. .- .-.- -.-. .... . .-. .-. -.-- .-.- .--. . .- .-.
```
*It's important to provide either translation flag too so that the tool knows which translation operation to perform.*

Finally, to store the output of any file translation, be input from a file or a string provided through the cli, you can do so providing a valid path to the `-o` flag like so:

`secret_message.txt`

```txt
-- --- .-. ... . / -.-. --- -.. . / .. ... / .- / - . .-.. . -.-. --- -- -- ..- -. .. -.-. .- - .. --- -. ... / -- . - .... --- -.. / .-- .... .. -.-. .... / . -. -.-. --- -.. . ... / - . -..- - / -.-. .... .- .-. .- -.-. - . .-. ... / .- ... / ... - .- -. -.. .- .-. -.. .. --.. . -.. / ... . --.- ..- . -. -.-. . ... / --- ..-. / - .-- --- / -.. .. ..-. ..-. . .-. . -. - / ... .. --. -. .- .-.. / -.. ..- .-. .- - .. --- -. ... --..-- / -.-. .- .-.. .-.. . -.. / -.. --- - ... / .- -. -.. / -.. .- ... .... . ... --..-- / --- .-. / -.. .. - ... / .- -. -.. / -.. .- .... ... .-.-.-
```

```sh
morse-code-translator -t -f secret_message.txt -o decoded.txt
```

`decoded.txt`
```txt
MORSE CODE IS A TELECOMMUNICATIONS METHOD WHICH ENCODES TEXT CHARACTERS AS STANDARDIZED SEQUENCES OF TWO DIFFERENT SIGNAL DURATIONS, CALLED DOTS AND DASHES, OR DITS AND DAHS.
```

## Reference

|Letter|Morse|
|------|-----|
|A| ".-"|
|B| "-..."|
|C| "-.-."|
|D| "-.."|
|E| "."|
|F| "..-."|
|G| "--."|
|H| "...."|
|I| ".."|
|J| ".---"|
|K| "-.-"|
|L| ".-.."|
|M| "--"|
|N| "-."|
|O| "---"|
|P| ".--."|
|Q| "--.-"|
|R| ".-."|
|S| "..."|
|T| "-"|
|U| "..-"|
|V| "...-"|
|W| ".--"|
|X| "-..-"|
|Y| "-.--"|
|Z| "--.."|

|Digit|Morse|
|------|-----|
|0| "----"|
|1| ".---"|
|2| "..---"|
|3| "...--"|
|4| "....-"|
|5| "....."|
|6| "-...."|
|7| "--..."|
|8| "---.."|
|9| "----."|


|Punctuation Mark|Morse|
|------|-----|
|\\ | "......"|
|&| ".-..."|
|'| ".----."|
|@| ".--.-."|
|)| "-.--.-"|
|(| "-.--."|
|:| "---..."|
|,| "--..--"|
|=| "-...-"|
|!| "-.-.--"|
|.| ".-.-.-"|
|-| "-....-"|
|+| ".-.-."|
|"| ".-..-."|
|?| "..--.."|
|/| "-..-."|
|\n| ".-.-"|
|[SPACE]| "/"|

## Roadmap
- GUI
- Play morse code sound
- Translate audio files from morse code to text