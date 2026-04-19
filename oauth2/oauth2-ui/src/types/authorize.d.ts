type AuthorizeScope = "user" | "read:user" | "user:email";

type Authorize = {
  id: string;
  application_id: string;
  user_id: string;
  scope: AuthorizeScope;
  created_at: Date;
};
