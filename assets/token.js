const USER_COOKIE = "_fq_user";

/**
 * John S
 * https://stackoverflow.com/questions/10730362/get-cookie-by-name
 */
function getCookie(name) {
  function escape(s) {
    return s.replace(/([.*+?^$(){}|\[\]\/\\])/g, '\\$1');
  }

  const match = document.cookie.match(RegExp('(?:^|;\\s*)' + escape(name) + '=([^;]*)'));
  return match ? match[1] : null;
}

async function fetchToken() {
  const response = await fetch("/auth", {
    method: "GET", headers: {"Content-Type": "application/json"},
  });

  return response.json();
}

/**
 * If there's already a token in cookies, use it.
 * Otherwise, fetch a new token from the server.
 */
export const TOKEN = getCookie(USER_COOKIE) || fetchToken()
  .then(data => {
    console.log("token recv:", data.token);
    return data.token;
  })
  .catch(e => console.error("error fetching token:", e));
