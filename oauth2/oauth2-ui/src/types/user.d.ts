type UserStatus = "Pending" | "Active" | "Freeze";
type User = {
  id: string;
  username: string;
  email: string;
  status: UserStatus;
  created_at: string;
};
