export function addMessage(
  text:string
) {

  const box =
    document.getElementById(
      "messages"
    ) as HTMLTextAreaElement;

  box.value += text + "\n";

}
