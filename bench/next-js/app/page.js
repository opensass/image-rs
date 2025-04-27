import Image from "next/image";
import styles from "./page.module.css";

export default function Home() {
  const images = Array.from({ length: 10000 }, (_, i) => (
    <Image
      key={i.toFixed()}
      src={"/800.svg"}
      alt={`Photo ${i}`}
      width={400}
      height={600}
      layout="responsive"
      quality={100}
      placeholder="blur"
      blurDataURL="https://placehold.co/800?text=Hello+World&font=roboto"
      priority={i < 5}
      className={styles.benchmarkImage}
    />
  ));

  return (
    <div className={styles.imageGrid}>
      {images}
    </div>
  );
}
