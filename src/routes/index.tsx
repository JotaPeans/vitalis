import { createFileRoute, Link } from "@tanstack/react-router";
import { Settings } from "lucide-react";

import PageHeader from "@/components/PageHeader";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";

import CleanMemoryButton from "./-components/CleanMemoryButton";
import DiskUsageBar from "./-components/DiskUsageBar";
import MemoryCard from "./-components/MemoryCard";
import SectionReveal from "./-components/SectionReveal";
import TemperatureSection from "./-components/TemperatureSection";
import TopAppsList from "./-components/TopAppsList";

export const Route = createFileRoute("/")({
  component: DashboardPage,
});

function DashboardPage() {
  return (
    <>
      <PageHeader
        title="Vitalis"
        subtitle="Atualizado a cada 2 segundos"
        actions={
          <Button variant="ghost" size="icon" asChild>
            <Link to="/settings" aria-label="Configurações">
              <Settings />
            </Link>
          </Button>
        }
      />
      <ScrollArea className="min-h-0 flex-1">
        <div className="flex flex-col gap-3 px-4 pb-3">
          <SectionReveal order={0}>
            <TemperatureSection />
          </SectionReveal>
          <SectionReveal order={1}>
            <MemoryCard />
          </SectionReveal>
          <SectionReveal order={2}>
            <DiskUsageBar />
          </SectionReveal>
          <SectionReveal order={3}>
            <TopAppsList />
          </SectionReveal>
        </div>
      </ScrollArea>
      <CleanMemoryButton />
    </>
  );
}
