static uint8_t __attribute__((section(".ccmram"))) adc_dma_buffer[256];

void start_adc_dma(void)
{
    HAL_ADC_Start_DMA(&hadc1, adc_dma_buffer, 256);
}
