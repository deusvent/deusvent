// CloudFront doesn't provide a simple way to redirect from the naked domain
// so we use <link rel="canonical"> with this hack which should be enough
const isNakedDomain = window.location.hostname.toLowerCase() == "deusvent.com";
if (isNakedDomain) {
  const redirect = window.location.href
    .toLowerCase()
    .replace("deusvent.com", "www.deusvent.com");
  window.location.replace(redirect);
}
