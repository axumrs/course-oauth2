import { $post, $get, $delete } from "@/fetch";

type ApplicationNewForm = {
  name: string;
  homepage: string;
  desc: string;
  callback: string;
};

type applicationSecretNewForm = {
  application_id: string;
};

const newApplicationApi = (data: ApplicationNewForm) =>
  $post<Application>("/api/application", data);

const applicationListApi = () => $get<Application[]>("/api/application");
const applicationDetailApi = (id: string) => () =>
  $get<Application>(`/api/application/${id}`);

const applicationSecretListApi = (id: string) => () =>
  $get<ApplicationSecret[]>(`/api/application/secret/${id}`);

const newApplicationSecretApi = (data: applicationSecretNewForm) =>
  $post<ApplicationSecret>(`/api/application/secret`, data);

const deleteApplicationSecretApi = (application_id: string, id: string) => () =>
  $delete(`/api/application/secret/${application_id}/${id}`);

export {
  newApplicationApi,
  applicationListApi,
  applicationDetailApi,
  applicationSecretListApi,
  newApplicationSecretApi,
  deleteApplicationSecretApi,
};
