type LoginForm = {
  username: string;
  password: string;
};
type RegisterForm = {
  username: string;
  password: string;
  email: string;
  re_password: string;
};
import { $post } from "@/fetch";

const loginApi = (data: LoginForm) => $post<Token>("/api/auth/login", data);
const registerApi = (data: RegisterForm) =>
  $post<any>("/api/auth/register", data);

export { loginApi, registerApi };
