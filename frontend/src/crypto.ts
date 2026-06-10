export async function encrypt(
  text:string
) {

  return btoa(text);

}

export async function decrypt(
  text:string
) {

  return atob(text);

}
