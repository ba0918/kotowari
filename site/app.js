function applyLanguage(language) {
  const copy = messages[language];
  document.documentElement.lang = language;
  document.title = copy.pageTitle;
  document.querySelector('meta[name="description"]').content = copy.description;
  document.querySelectorAll('[data-i18n]').forEach(element => {
    element.textContent = copy[element.dataset.i18n];
  });
  document.querySelectorAll('[data-language]').forEach(button => {
    button.setAttribute('aria-pressed', String(button.dataset.language === language));
  });
}

document.querySelectorAll('[data-language]').forEach(button => {
  button.addEventListener('click', () => {
    applyLanguage(button.dataset.language);
    siteLanguage.save(button.dataset.language);
  });
});

applyLanguage(siteLanguage.read());
