import { Composition } from "remotion";
import { ScanCamTour, TOTAL_FRAMES } from "./compositions/Tour";

export const RemotionRoot: React.FC = () => {
  return (
    <Composition
      id="ScanCamTour"
      component={ScanCamTour}
      durationInFrames={TOTAL_FRAMES}
      fps={30}
      width={1280}
      height={800}
    />
  );
};
