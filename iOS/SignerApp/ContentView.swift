import SharedTypes
import SwiftUI
import web3

enum Outcome {
    case signer(SignerResponse)
}

typealias Uuid = [UInt8]

enum Message {
    case event(Event)
    case response(Uuid, Outcome)
}

@MainActor
class Model: ObservableObject {
    @Published var view = ViewModel(text: "")
    var account:EthereumAccount
    
    init() {
        let keyStorage = EthereumKeyLocalStorage()
        account = try! EthereumAccount.create(replacing: keyStorage, keystorePassword: "Password")
    }

    private func sign_message(uuid:Uuid,msg:String) {
        Task{
            self.update(msg: .response(uuid, .signer(.signature(Array(try! account.sign(message: msg.data(using: .utf8)!))))))
        }
    }

    func update(msg: Message) {
        let reqs: [Request]

        switch msg {
        case let .event(m):
            reqs = try! [Request].bincodeDeserialize(input: SignerApp.processEvent(try! m.bincodeSerialize()))
        case let .response(uuid, outcome):
            reqs = try! [Request].bincodeDeserialize(input: SignerApp.handleResponse(uuid, {
                switch outcome {
                case let .signer(x):
                    return try! x.bincodeSerialize()
                }
            }()))
        }

        for req in reqs {
            switch req.effect {
            case .render: view = try! ViewModel.bincodeDeserialize(input: SignerApp.view())
            case let .signer(r): sign_message(uuid:req.uuid,msg: r.message)
            }
        }
    }
}

struct ActionButton: View {
    var label: String
    var color: Color
    var action: () -> Void

    init(label: String, color: Color, action: @escaping () -> Void) {
        self.label = label
        self.color = color
        self.action = action
    }

    var body: some View {
        Button(action: action) {
            Text(label)
                .fontWeight(.bold)
                .font(.body)
                .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                .background(color)
                .cornerRadius(10)
                .foregroundColor(.white)
                .padding()
        }
    }
}

struct ContentView: View {
    @ObservedObject var model: Model
    @State private var username: String = ""


    var body: some View {
        VStack {
            Text("Crux Counter Example").font(.headline)
            Text("Rust Core, Swift Shell (SwiftUI)").padding()
            Text(String(model.view.text))
                .foregroundColor(Color.gray)
                .padding()
            Text(String(username))
            Form {
                TextField("message:",text: $username,onCommit: {model.update(msg: .event(.signMessage(username)))})
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(model: Model())
    }
}
