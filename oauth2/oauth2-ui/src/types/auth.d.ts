type TokenKind = "Token" | "AccessToken" | "TempAccessToken";
type Token = {
  id: string;
  user_id: string;
  token: string;
  kind: TokenKind;
  application_id: string;
  created_at: string;
  expired_at: string;
};
