import { $post, $get } from "@/fetch";

type NewAuthorizeForm = {
  client_id: string;
  scope: string;
};
type AccessTokenForm = {
  client_id: string;
  client_secret: string;
  code: string;
};

const findAuthorizeApi = (id: string) =>
  $get<Authorize & { is_authorized: boolean }>(
    `/api/login/oauth/authorize/${id}`,
  );

const newAuthorizeApi = (data: NewAuthorizeForm) => () =>
  $post<Token>("/api/login/oauth/authorize", data);

const accessTokenApi = (data: AccessTokenForm) =>
  $post<Token>(`/api/login/oauth/access_token`, data);

export { findAuthorizeApi, newAuthorizeApi, accessTokenApi };
