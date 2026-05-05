['assets/svg/tofa.svg', 'assets/svg/issuers.svg'].forEach(function (url) {
  fetch(url)
    .then(function (r) { return r.text(); })
    .then(function (svg) {
      var div = document.createElement('div');
      div.style.cssText = 'position:absolute;width:0;height:0;overflow:hidden';
      div.setAttribute('aria-hidden', 'true');
      div.innerHTML = svg;
      document.body.insertBefore(div, document.body.firstChild);
    });
});
