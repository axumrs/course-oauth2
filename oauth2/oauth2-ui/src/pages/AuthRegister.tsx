import AuthRegisterForm from "@/components/form/AuthRegisterForm";
import { useState } from "react";
import { CheckCircleIcon } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Link } from "react-router-dom";

export default function AuthRegister() {
  const [isRegisterSuccess, setIsRegisterSuccess] = useState(false);
  return (
    <>
      {isRegisterSuccess ? (
        <RegisterSuccess />
      ) : (
        <AuthRegisterForm successCallback={() => setIsRegisterSuccess(true)} />
      )}
    </>
  );
}

function RegisterSuccess() {
  return (
    <div className="flex flex-col justify-center items-center">
      <Alert className=" border-green-200 bg-green-50 text-green-900 dark:border-green-900 dark:bg-green-950 dark:text-green-50">
        <CheckCircleIcon />
        <AlertTitle>注册成功</AlertTitle>
        <AlertDescription>
          🎉 恭喜，注册成功。请<Link to="/auth/login">登录</Link>你的账号。
        </AlertDescription>
      </Alert>
    </div>
  );
}
