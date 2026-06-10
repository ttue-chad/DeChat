export async function saveKey(
  key:string,
  value:string
) {

  localStorage.setItem(key, value);

}

export async function loadKey(
  key:string
) {

  return localStorage.getItem(key);

}
