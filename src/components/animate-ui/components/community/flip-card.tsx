"use client";

import { easeOut, motion } from "motion/react";
import * as React from "react";

export interface FlipCardData {
  name: string;
  image: string;
  bio: string;
}

interface FlipCardProps {
  data: FlipCardData;
  children?: React.ReactNode;
}

export function FlipCard({ data, children }: FlipCardProps) {
  const [isFlipped, setIsFlipped] = React.useState(false);

  const isTouchDevice =
    typeof window !== "undefined" && "ontouchstart" in window;

  const handleClick = () => {
    if (isTouchDevice) setIsFlipped(!isFlipped);
  };

  const handleMouseEnter = () => {
    if (!isTouchDevice) setIsFlipped(true);
  };

  const handleMouseLeave = () => {
    if (!isTouchDevice) setIsFlipped(false);
  };

  const cardVariants = {
    front: { rotateY: 0, transition: { duration: 0.5, ease: easeOut } },
    back: { rotateY: 180, transition: { duration: 0.5, ease: easeOut } },
  };

  return (
    <div
      className="mt-2 relative w-full min-h-52 perspective-1000 cursor-pointer mx-auto"
      onClick={handleClick}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* FRONT: Profile */}
      <motion.div
        className="absolute inset-0 backface-hidden  flex flex-col items-center justify-center "
        animate={isFlipped ? "back" : "front"}
        variants={cardVariants}
        style={{ transformStyle: "preserve-3d" }}
      >
        <img
          src={data.image}
          alt={data.name}
          className="w-full h-full  object-cover"
        />
      </motion.div>

      {/* BACK: Bio + Stats + Socials */}
      <motion.div
        className="absolute inset-0 backface-hidden p-4 flex flex-col justify-between items-center gap-y-4  from-muted via-background to-muted "
        initial={{ rotateY: 180 }}
        animate={isFlipped ? "front" : "back"}
        variants={cardVariants}
        style={{
          transformStyle: "preserve-3d",
          rotateY: 180,
          backgroundImage: `url(${data.image})`,
          backdropFilter: "blur(8px)",
          backgroundColor: "rgba(0, 0, 0, 0.7)",
          backgroundSize: "cover",
          backgroundPosition: "center",
          backgroundBlendMode: "overlay",
        }}
      >
        {children ? <div className="w-full h-full">{children}</div> : null}
      </motion.div>
    </div>
  );
}
