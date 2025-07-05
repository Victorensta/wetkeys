import { useEffect, useState } from "react";
import FileUpload from "./components/FileUpload";
import FileList from "./components/FileList";
import { vtk_backend as defaultActor } from "declarations/vtk_backend";
import { createActor, canisterId } from "declarations/vtk_backend";
import { AuthClient } from "@dfinity/auth-client";
import { Button } from "@/components/ui/button";

const identityProvider =
  process.env.DFX_NETWORK === "ic"
    ? "https://identity.ic0.app"
    : "http://uxrrr-q7777-77774-qaaaq-cai.localhost:4943/";

    function App() {
  const [greeting, setGreeting] = useState("");
  const [actor, setActor] = useState(defaultActor);
  const [authClient, setAuthClient] = useState(null);
  const [principal, setPrincipal] = useState("");
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  useEffect(() => {
    AuthClient.create().then(async (client) => {
      const identity = client.getIdentity();
      const newActor = createActor(canisterId, {
        agentOptions: { identity }
      });
      setAuthClient(client);
      setActor(newActor);
      setIsAuthenticated(await client.isAuthenticated());
    });
  }, []);

  const login = async () => {
    await authClient.login({
      identityProvider,
      onSuccess: async () => {
        const identity = authClient.getIdentity();
        const authedActor = createActor(canisterId, {
          agentOptions: { identity }
        });
        setActor(authedActor);
        setIsAuthenticated(true);
      }
    });
  };

  const logout = async () => {
    await authClient.logout();
    const anonActor = createActor(canisterId); // fallback to anonymous
    setActor(anonActor);
    setIsAuthenticated(false);
    setPrincipal("");
  };

  const whoami = async () => {
    const result = await actor.whoami();
    // console.log(Object.keys(actor)); //logs method names available on the actor object 
    setPrincipal(result.toString());
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    const name = event.target.elements.name.value;
    actor.greet(name).then(setGreeting);
  };

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>

      <section id="greeting">{greeting}</section>
      <FileUpload actor={actor} />
      {isAuthenticated && <FileList actor={actor} />}
      <div class="bg-blue-500 text-white text-xl p-4 rounded">✅ Tailwind is working!</div>
      <Button>Click me</Button>

      {/* 🔑 Internet Identity Controls */}
      <div>
        {!isAuthenticated ? (
          <Button onClick={login}>Login</Button>
        ) : (
          <Button onClick={logout}>Logout</Button>
        )}
        <Button onClick={whoami}>Whoami</Button>
      </div>

      {principal && (
        <div>
          <h2>Your principal ID is:</h2>
          <p>{principal}</p>
        </div>
      )}
    </main>
  );
}

export default App;
