import Container from "@/components/Container";
import AuthLoginForm from "@/components/form/AuthLoginForm";
import OauthAuthorizeConfirm from "@/components/OauthAuthorizeConfirm";
import { useNavigate } from "react-router-dom";

export default function OauthAuthorize() {
  const nav = useNavigate();
  return (
    <Container className="max-w-md">
      {window.sessionStorage.getItem("token") ? (
        <OauthAuthorizeConfirm />
      ) : (
        <AuthLoginForm classname="my-6" callback={() => nav(0)} />
      )}
    </Container>
  );
}
