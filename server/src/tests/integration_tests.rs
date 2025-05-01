#[cfg(test)]
mod tests {}

//  /api/speech działa z JSON-em { "text": "Hello" }
//  /api/speech działa z JSON-em { "file_path": "test.txt" }
//  /api/speech zwraca 400 przy nieistniejącym pliku
//  /api/speech zwraca 500 przy błędzie TTS
//  /api/healthcheck zwraca 200
