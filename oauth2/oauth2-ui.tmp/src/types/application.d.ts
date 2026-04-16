type ApplicationStatus = "Pending" | "Active" | "Reject";
type Application = {
  id: string;
  user_id: string;
  name: string;
  description: string;
  homepage_url: string;
  callback_url: string;
  created_at: string;
  updated_at: string;
  status: ApplicationStatus;
};
