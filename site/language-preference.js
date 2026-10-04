const siteLanguage = {
  read() {
    try {
      const saved = localStorage.getItem('ba0918-language');
      if (saved === 'en' || saved === 'ja') return saved;
    } catch {
      // Browsers may deny storage; the current page must still work.
    }
    return 'en';
  },
  save(language) {
    if (language !== 'en' && language !== 'ja') return;
    try {
      localStorage.setItem('ba0918-language', language);
    } catch {
      // Persistence is optional; applying the choice is not.
    }
  }
};
