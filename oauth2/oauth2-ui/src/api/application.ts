import { $post, $get } from "@/fetch";

type ApplicationNewForm = {
  name: string;
  homepage: string;
  desc: string;
  callback: string;
};

const newApplicationApi = (data: ApplicationNewForm) =>
  $post<Application>("/api/application", data);

const applicationListApi = () => $get<Application[]>("/api/application");

export { newApplicationApi, applicationListApi };
