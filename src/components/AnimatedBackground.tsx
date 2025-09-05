import React from "react";

export function AnimatedBackground() {
  // Generate an array of particles with random properties
  const particles = Array.from({ length: 250 }, (_, i) => ({
    id: i,
    size: Math.random() * 7 + 1, // 1-4px
    left: Math.random() * 100, // 0-100%
    animationDelay: Math.random() * 20, // 0-20s
    animationDuration: 20 + Math.random() * 20, // 20-40s
    opacity: Math.random() * 0.3 + 0.05, // 0.05-0.35
  }));

  // Generate glitter particles for the top section
  const glitterParticles = Array.from(
    { length: 256 },
    (_, i) => ({
      id: i,
      size: Math.random() * 2 + 0.1, // 0.5-2.5px - very small
      left: Math.random() * 100, // 0-100%
      top: Math.random() * 45, // 0-40% from top
      animationDelay: Math.random() * 10, // 0-10s
      animationDuration: 8 + Math.random() * 12, // 8-20s
      opacity: Math.random() * 0.5 + 0.2, // 0.2-0.8
      glowIntensity: Math.random() * 0.5 + 0.3, // 0.2-0.7
    }),
  );

  return (
    <div className="fixed inset-0 overflow-hidden pointer-events-none z-0">
      {/* Background gradient - dark navy/teal */}
      <div className="absolute inset-0 bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950" />

      {/* Top gradient overlay with green tint - inspired by Nuxt */}
      <div
        className="absolute inset-0 bg-gradient-to-b from-emerald-950/40 via-emerald-950/10 to-transparent"
        style={{ height: "60%" }}
      />

      {/* Glitter particles at the top */}
      {glitterParticles.map((particle) => (
        <div
          key={`glitter-${particle.id}`}
          className="absolute rounded-full bg-emerald-300 animate-glitter-float"
          style={{
            width: `${particle.size}px`,
            height: `${particle.size}px`,
            left: `${particle.left}%`,
            top: `${particle.top}%`,
            opacity: particle.opacity,
            animationDelay: `${particle.animationDelay}s`,
            animationDuration: `${particle.animationDuration}s`,
            boxShadow: `0 0 ${particle.glowIntensity * 4}px rgba(52, 211, 153, ${particle.glowIntensity})`,
            filter: `blur(${particle.size * 0.1}px)`,
          }}
        />
      ))}

      {/* Floating particles - subtle green tint */}
      {particles.map((particle) => (
        <div
          key={particle.id}
          className="absolute rounded-full bg-emerald-400/30 backdrop-blur-sm animate-float"
          style={{
            width: `${particle.size}px`,
            height: `${particle.size}px`,
            left: `${particle.left}%`,
            top: "100%",
            opacity: particle.opacity,
            animationDelay: `${particle.animationDelay}s`,
            animationDuration: `${particle.animationDuration}s`,
          }}
        />
      ))}

      {/* Subtle grid pattern with green tint */}
      <div
        className="absolute inset-0 opacity-3 animate-grid-move"
        style={{
          backgroundImage: `
            linear-gradient(rgba(52, 211, 153, 0.08) 1px, transparent 1px),
            linear-gradient(90deg, rgba(52, 211, 153, 0.08) 1px, transparent 1px)
          `,
          backgroundSize: "50px 50px",
        }}
      />
    </div>
  );
}