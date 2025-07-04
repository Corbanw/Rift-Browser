List<String> extractImages(String html) {
  try {
    final images = <String>[];
    final imgPattern1 = RegExp(r'<img[^>]+src="([^"]+)"[^>]*>', caseSensitive: false);
    final imgPattern2 = RegExp(r"<img[^>]+src='([^']+)'[^>]*>", caseSensitive: false);
    final matches1 = imgPattern1.allMatches(html);
    final matches2 = imgPattern2.allMatches(html);
    for (final match in matches1) {
      final src = match.group(1);
      if (src != null && src.isNotEmpty) {
        images.add(src);
      }
    }
    for (final match in matches2) {
      final src = match.group(1);
      if (src != null && src.isNotEmpty) {
        images.add(src);
      }
    }
    return images;
  } catch (e) {
    return [];
  }
} 