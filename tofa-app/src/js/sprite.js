fetch('assets/svg/sprite.svg')
  .then(function (r) { return r.text(); })
  .then(function (svg) {
    var div = document.createElement('div');
    div.style.cssText = 'position:absolute;width:0;height:0;overflow:hidden';
    div.setAttribute('aria-hidden', 'true');
    div.innerHTML = svg;
    document.body.insertBefore(div, document.body.firstChild);
  });
