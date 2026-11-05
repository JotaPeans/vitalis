import { Outlet, createRootRouteWithContext, useLocation } from "@tanstack/react-router";
import type { QueryClient } from "@tanstack/react-query";
import { AnimatePresence, motion } from "motion/react";
import { Toaster } from "sonner";
import { useTheme } from "next-themes";

import { useVitalsSubscription } from "@/hooks/use-vitals-subscription";

interface RouterContext {
  queryClient: QueryClient;
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: RootLayout,
});

function RootLayout() {
  useVitalsSubscription();
  const { pathname } = useLocation();
  const { resolvedTheme } = useTheme();

  return (
    <div className="bg-background flex h-full flex-col overflow-hidden rounded-xl">
      <AnimatePresence mode="wait" initial={false}>
        <motion.main
          key={pathname}
          className="flex min-h-0 flex-1 flex-col"
          initial={{ opacity: 0, x: pathname === "/" ? -12 : 12 }}
          animate={{ opacity: 1, x: 0 }}
          exit={{ opacity: 0, x: pathname === "/" ? -12 : 12 }}
          transition={{ duration: 0.16, ease: "easeOut" }}
        >
          <Outlet />
        </motion.main>
      </AnimatePresence>
      <Toaster
        position="bottom-center"
        theme={resolvedTheme === "dark" ? "dark" : "light"}
        toastOptions={{ className: "!text-xs !rounded-lg" }}
      />
    </div>
  );
}
