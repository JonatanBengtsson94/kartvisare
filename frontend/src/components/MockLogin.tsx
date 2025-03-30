import { mockLogin } from "../services/api";

const MockLogin = () => {
  const handleLogin = async () => {
    try {
      await mockLogin();
      alert("Login successful!");

      const redirectUrl = localStorage.getItem("redirectAfterLogin") || "/";
      window.location.href = redirectUrl;
    } catch (error) {
      alert("Login failed!");
    }
  };

  return (
    <div>
      <button onClick={handleLogin}>Login</button>
    </div>
  );
};

export default MockLogin;
