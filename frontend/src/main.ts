import { addMessage } from "./ui";
import { encrypt } from "./crypto";

const sendBtn =
  document.getElementById("sendBtn")!;

const input =
  document.getElementById(
    "messageInput"
  ) as HTMLInputElement;

sendBtn.addEventListener(
  "click",
  async () => {

    const encrypted =
      await encrypt(input.value);

    addMessage(
      "You: " + input.value
    );

    console.log(
      encrypted
    );

    input.value = "";

  }
);
