List<String> extractLinks(String html) {
  try {
    final links = <String>[];
    final linkPattern1 = RegExp(r'<a[^>]+href="([^"]+)"[^>]*>', caseSensitive: false);
    final linkPattern2 = RegExp(r"<a[^>]+href='([^']+)'[^>]*>", caseSensitive: false);
    final matches1 = linkPattern1.allMatches(html);
    final matches2 = linkPattern2.allMatches(html);
    for (final match in matches1) {
      final href = match.group(1);
      if (href != null && href.isNotEmpty && !href.startsWith('#')) {
        links.add(href);
      }
    }
    for (final match in matches2) {
      final href = match.group(1);
      if (href != null && href.isNotEmpty && !href.startsWith('#')) {
        links.add(href);
      }
    }
    return links;
  } catch (e) {
    return [];
  }
} 