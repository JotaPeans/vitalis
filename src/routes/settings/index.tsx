import { createFileRoute } from "@tanstack/react-router";

import PageHeader from "@/components/PageHeader";
import { ScrollArea } from "@/components/ui/scroll-area";

import AutostartCard from "./-components/AutostartCard";
import ProtectedAppsCard from "./-components/ProtectedAppsCard";

export const Route = createFileRoute("/settings/")({
  component: SettingsPage,
});

function SettingsPage() {
  return (
    <>
      <PageHeader title="Configurações" backTo="/" />
      <ScrollArea className="min-h-0 flex-1">
        <div className="flex flex-col gap-3 px-4 pb-4">
          <AutostartCard />
          <ProtectedAppsCard />
        </div>
      </ScrollArea>
    </>
  );
}
