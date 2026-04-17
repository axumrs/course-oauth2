import NewApplicationForm from "@/components/form/NewApplicationForm";
import PageTitle from "@/components/PageTitle";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";

export default function NewApplication() {
  return (
    <>
      <div className="flex justify-between items-center gap-x-3 my-3">
        <PageTitle>创建应用</PageTitle>
        <div>
          <Button variant="secondary" asChild>
            <Link to="/application">应用列表</Link>
          </Button>
        </div>
      </div>
      <NewApplicationForm />
    </>
  );
}
