export class ChatPeer {

  private pc: RTCPeerConnection;
  private channel?: RTCDataChannel;

  constructor() {

    this.pc = new RTCPeerConnection({
      iceServers: [
        {
          urls: "stun:stun.l.google.com:19302"
        }
      ]
    });

  }

  createChannel() {

    this.channel =
      this.pc.createDataChannel("chat");

    return this.channel;
  }

  onMessage(
    callback:(msg:string)=>void
  ) {

    this.pc.ondatachannel = (event) => {

      const channel = event.channel;

      channel.onmessage = (msg) => {

        callback(msg.data);

      };

    };

  }

}
