[
  'assets/svg/tofa/wink.svg',
  'assets/svg/tofa/open.svg',
  'assets/svg/tofa/wink-mono.svg',
  'assets/svg/issuers/adobe.svg',
  'assets/svg/issuers/amazonaws.svg',
  'assets/svg/issuers/apple.svg',
  'assets/svg/issuers/atlassian.svg',
  'assets/svg/issuers/bitbucket.svg',
  'assets/svg/issuers/bitwarden.svg',
  'assets/svg/issuers/cloudflare.svg',
  'assets/svg/issuers/coinbase.svg',
  'assets/svg/issuers/digitalocean.svg',
  'assets/svg/issuers/discord.svg',
  'assets/svg/issuers/docker.svg',
  'assets/svg/issuers/dropbox.svg',
  'assets/svg/issuers/facebook.svg',
  'assets/svg/issuers/github.svg',
  'assets/svg/issuers/gitlab.svg',
  'assets/svg/issuers/google.svg',
  'assets/svg/issuers/heroku.svg',
  'assets/svg/issuers/linkedin.svg',
  'assets/svg/issuers/mailchimp.svg',
  'assets/svg/issuers/microsoft.svg',
  'assets/svg/issuers/notion.svg',
  'assets/svg/issuers/npm.svg',
  'assets/svg/issuers/paypal.svg',
  'assets/svg/issuers/protonmail.svg',
  'assets/svg/issuers/reddit.svg',
  'assets/svg/issuers/shopify.svg',
  'assets/svg/issuers/slack.svg',
  'assets/svg/issuers/stripe.svg',
  'assets/svg/issuers/twitch.svg',
  'assets/svg/issuers/vercel.svg',
  'assets/svg/issuers/wordpress.svg',
  'assets/svg/issuers/x.svg',
  'assets/svg/issuers/authelia.svg',
  'assets/svg/issuers/ovh.svg',
].forEach(function (url) {
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
