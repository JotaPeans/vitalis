import { motion, type HTMLMotionProps } from "motion/react";

import { SECTION_STAGGER_SECONDS } from "@/lib/constants";

interface SectionRevealProps extends HTMLMotionProps<"section"> {
  order: number;
}

const SectionReveal = ({ order, transition, ...props }: SectionRevealProps) => {
  return (
    <motion.section
      initial={{ opacity: 0, y: 6 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.22, ease: "easeOut", delay: order * SECTION_STAGGER_SECONDS, ...transition }}
      {...props}
    />
  );
};

export default SectionReveal;
